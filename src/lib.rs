use tonic::Status;
use hello_world::{HelloReply, HelloRequest};
use hello_world::greeter_server::{Greeter};

pub mod hello_world {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/rust/helloworld.rs"));
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<tonic::Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(tonic::Response::new(reply)) // Send back our formatted greeting
    }
}