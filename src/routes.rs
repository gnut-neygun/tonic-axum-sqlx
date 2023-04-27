use std::collections::HashMap;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tonic::{Request, Response, Status};

use tonic_axum_sqlx::generated::object_api::{Object, ObjectId, ObjectList};
use tonic_axum_sqlx::generated::object_api::object_api_server::ObjectApi;

use crate::utils::{ResponseStream, TonicResponse};

// gRPC Routes
#[derive(Debug, Default)]
pub struct ObjectService {}

#[tonic::async_trait]
impl ObjectApi for ObjectService {
    async fn list_objects(&self, request: Request<()>) -> Result<Response<ObjectList>, Status> {
        let my_list = vec![{
                               Object {
                                   id: 1,
                                   name: String::from("apple"),
                                   properties: HashMap::new(),
                               }
                           }, {
                               Object {
                                   id: 2,
                                   name: String::from("banana"),
                                   properties: HashMap::new(),
                               }
                           }];
        let reply = ObjectList {
            objects: my_list
        };
        Ok(Response::new(reply))
    }

    async fn get_object(&self, request: Request<ObjectId>) -> Result<Response<Object>, Status> {
        let my_object = Object {
            id: 1,
            name: String::from("apple"),
            properties: HashMap::new(),
        };
        Ok(Response::new(my_object))
    }

    async fn create_object(&self, request: Request<Object>) -> Result<Response<Object>, Status> {
        todo!()
    }

    async fn update_object(&self, request: Request<Object>) -> Result<Response<Object>, Status> {
        todo!()
    }

    async fn delete_object(&self, request: Request<ObjectId>) -> Result<Response<()>, Status> {
        todo!()
    }

    type subscribe_objectStream = ResponseStream<Object>;

    async fn subscribe_object(&self, request: Request<ObjectId>) -> Result<Response<Self::subscribe_objectStream>, Status> {
        todo!()
    }
}

// Rest Routes
pub fn object_route() -> Router {
    Router::new()
        .route("/objects", get(list_objects))
        .route("/object/:id", get(get_object))
}

async fn list_objects() -> impl IntoResponse {
    let object_service = ObjectService {};
    let grpc_request = Request::new(());
    let grpc_response = object_service.list_objects(grpc_request).await;
    grpc_response.map(|res| TonicResponse::from(res)).map_err(|status| status.to_string())
}

async fn get_object(Path(id): Path<u64>) -> impl IntoResponse {
    let object_service = ObjectService {};
    let grpc_request = Request::new(ObjectId { id });
    let grpc_response = object_service.get_object(grpc_request).await;
    grpc_response.map(|res| TonicResponse::from(res)).map_err(|status| status.to_string())
}

async fn create_object() -> impl IntoResponse {
    todo!()
}

async fn update_object() -> impl IntoResponse {
    todo!()
}
