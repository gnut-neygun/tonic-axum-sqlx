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
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use routes::ObjectService;
use tonic_axum_sqlx::object_api::object_api_server::ObjectApiServer;

use crate::grpc_rest_multiplex::MultiplexService;

mod grpc_rest_multiplex;
/// This module contains routing for gRPC and REST Service
mod routes;
mod utils;

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

    let project_root_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Setup tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let axum_service = axum::Router::new()
        .nest("/api", routes::object_route())
        // Serve the static test web site that calls the GRPC
        .nest_service(
            "/static",
            get_service(ServeDir::new(project_root_path.join("assets"))),
        )
        .route("/", get(swagger_ui))
        .route("/docs/openapi.yaml", get(openapi_doc))
        .with_state(shared_state);

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

    let hybrid_service = MultiplexService::new(axum_service, grpc_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let server = hyper::Server::bind(&addr).serve(tower::make::Shared::new(hybrid_service));

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
