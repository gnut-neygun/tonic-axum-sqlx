use std::net::SocketAddr;
use tonic_axum_sqlx::generated::helloworld::greeter_server::GreeterServer;
use tonic_axum_sqlx::MyGreeter;

mod axum_tonic_glue;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let greeter_service = GreeterServer::new(MyGreeter::default());

    let axum_service = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello world!" }))
        .into_make_service();

    let grpc_service = tonic::transport::Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(greeter_service))
        .into_service();

    let hybrid_service = axum_tonic_glue::make_hybrid_service(axum_service, grpc_service);

    let server = hyper::Server::bind(&addr).serve(hybrid_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}