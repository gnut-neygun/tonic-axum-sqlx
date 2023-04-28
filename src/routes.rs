use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use sqlx::{Pool, Postgres, query};
use tonic::{Request, Response, Status};
use tonic::Code::Aborted;

use tonic_axum_sqlx::generated::object_api::{Object, ObjectId, ObjectList};
use tonic_axum_sqlx::generated::object_api::object_api_server::ObjectApi;

use crate::{AppState, SharedState};
use crate::utils::{convert_to_hashmap, ResponseStream, TonicResponse};

// gRPC Routes
#[derive(Debug, Clone)]
pub struct ObjectService {
    pub(crate) db: Pool<Postgres>,
}

#[tonic::async_trait]
impl ObjectApi for ObjectService {
    async fn list_objects(&self, request: Request<()>) -> Result<Response<ObjectList>, Status> {
        println!("Got a request: {:?}", request);
        let my_list = query!("SELECT * FROM object")
            .fetch_all(&self.db)
            .await
            .map_err(|err|
                Status::new
                    (Aborted, err.to_string()))?
            .into_iter()
            .map(|record| {
                Object {
                    id: record.id as u64,
                    name: record.name.unwrap(),
                    properties: convert_to_hashmap(record.properties.unwrap()),
                }
            }).collect();
        Ok(Response::new(ObjectList { objects: my_list }))
    }

    async fn get_object(&self, request: Request<ObjectId>) -> Result<Response<Object>, Status> {
        println!("Got a request: {:?}", request);
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
pub fn object_route() -> Router<SharedState> {
    Router::new()
        .route("/objects", get(list_objects))
        .route("/object/:id", get(get_object))
}

async fn list_objects(State(state): State<SharedState>) -> Result<TonicResponse<ObjectList>,
    String> {
    let grpc_request = Request::new(());
    let grpc_response = state.object_service.list_objects(grpc_request).await;
    grpc_response.map(|res| TonicResponse::from(res)).map_err(|status| status.to_string())
}

async fn get_object(State(state): State<Arc<AppState>>, Path(id): Path<u64>) -> impl IntoResponse {
    let grpc_request = Request::new(ObjectId { id });
    let grpc_response = state.object_service.get_object(grpc_request).await;
    grpc_response.map(|res| TonicResponse::from(res)).map_err(|status| status.to_string())
}

async fn create_object(State(state): State<SharedState>) -> impl IntoResponse {
    todo!()
}

async fn update_object(State(state): State<SharedState>) -> impl IntoResponse {
    todo!()
}
