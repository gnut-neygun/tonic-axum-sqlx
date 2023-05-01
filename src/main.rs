#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::grpc_rest_multiplex::hybrid;
use routes::ObjectService;
use tonic_axum_sqlx::object_api::object_api_server::ObjectApiServer;

/// Logic for multiplexing grpc and normal webrequest based on header.
/// See https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part1/
mod grpc_rest_multiplex;
/// This module contains routing for gRPC and REST Service
mod routes;
mod utils;
mod websocket;

pub struct AppState {
    object_service: ObjectService,
}

pub type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:6000/postgres".to_string());
    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");
    let object_service = ObjectService { db: pool };

    let shared_state = Arc::new(AppState {
        object_service: object_service.clone(),
    });

    let trace_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(trace_subscriber).unwrap();

    let project_root_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let axum_factory_service = axum::Router::new()
        .merge(websocket::websocket_router())
        .nest("/api", routes::object_route())
        // Serve the static test web site that calls the GRPC
        .nest_service(
            "/static",
            get_service(ServeDir::new(project_root_path.join("assets"))),
        )
        .route("/", get(swagger_ui))
        .route("/docs/openapi.yaml", get(openapi_doc))
        .with_state(shared_state)
        .into_make_service_with_connect_info::<SocketAddr>();

    // Refer to https://github.com/tokio-rs/axum/tree/main/examples/rest-grpc-multiplex
    const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        concat!("/generated/rust/object_api_descriptor.bin")
    ));

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let grpc_service = tonic::transport::Server::builder()
        .accept_http1(true)
        .add_service(reflection_service)
        .add_service(tonic_web::enable(ObjectApiServer::new(object_service)))
        .into_service();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let hybrid_service = hybrid(axum_factory_service, grpc_service);
    let server = hyper::Server::bind(&addr).serve(hybrid_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

/// Router handler that responds with OpenAPI doc
async fn openapi_doc() -> impl IntoResponse {
    let api_doc = include_str!("../generated/openapi.yaml");
    api_doc
}

/// Router handler that responds with Swagger UI based on OpenAPI Schema
async fn swagger_ui() -> impl IntoResponse {
    let html_string = include_str!("../assets/swagger.html");
    axum::response::Html(html_string)
}
