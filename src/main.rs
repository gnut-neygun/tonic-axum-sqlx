#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::net::SocketAddr;

use axum::response::IntoResponse;
use axum::routing::get;

use routes::ObjectService;
use tonic_axum_sqlx::generated::object_api::object_api_server::ObjectApiServer;

mod grpc_rest_multiplex;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let object_service = ObjectApiServer::new(ObjectService::default());

    let axum_service = axum::Router::new()
        .nest("/api", routes::object_route())
        .route("/", get(swagger_ui))
        .route("/docs/openapi.yaml", get(openapi_doc))
        .into_make_service();

    let grpc_service = tonic::transport::Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(object_service))
        .into_service();

    let hybrid_service = grpc_rest_multiplex::make_hybrid_service(axum_service, grpc_service);

    let server = hyper::Server::bind(&addr).serve(hybrid_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn openapi_doc() -> impl IntoResponse {
    let api_doc = include_str!("../generated/openapi.yaml");
    api_doc
}

async fn swagger_ui() -> impl IntoResponse {
    let html_string = include_str!("../assets/swagger.html");
    axum::response::Html(html_string)
}