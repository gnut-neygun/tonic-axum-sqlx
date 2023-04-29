use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::{query, Pool, Postgres};
use tonic::Code::Aborted;
use tonic::{Request, Response, Status};
use tower_http::trace::TraceLayer;

use tonic_axum_sqlx::object_api::object_api_server::ObjectApi;
use tonic_axum_sqlx::object_api::{Object, ObjectId, ObjectList};

use crate::utils::{AxumResponse, ResponseStream};
use crate::{AppState, SharedState};

#[derive(Debug, Clone)]
pub struct ObjectService {
    pub(crate) db: Pool<Postgres>,
}

#[tonic::async_trait]
impl ObjectApi for ObjectService {
    async fn list_objects(&self, request: Request<()>) -> Result<Response<ObjectList>, Status> {
        let my_list = query!("SELECT * FROM object")
            .fetch_all(&self.db)
            .await
            .map_err(|err| Status::new(Aborted, err.to_string()))?
            .into_iter()
            .map(|record| Object {
                id: record.id as u64,
                name: record.name.unwrap(),
                properties: {
                    let map: HashMap<String, serde_json::Value> =
                        serde_json::from_value(record.properties.unwrap()).unwrap();
                    map.into_iter()
                        .map(|(key, value)| (key, value.to_string()))
                        .collect()
                },
            })
            .collect();
        Ok(Response::new(ObjectList { objects: my_list }))
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
        let object = request.into_inner();
        query!(
            r#"INSERT INTO object (name, properties) VALUES ($1, $2)"#,
            object.name,
            serde_json::to_value(&object).unwrap()
        )
        .execute(&self.db)
        .await
        .map_err(|err| Status::new(Aborted, err.to_string()))?;
        Ok(Response::new(object))
    }

    async fn update_object(&self, request: Request<Object>) -> Result<Response<Object>, Status> {
        let object = request.into_inner();
        query!(
            r#"UPDATE object SET name = $1, properties = $2 WHERE id = $3"#,
            object.name,
            serde_json::to_value(&object).unwrap(),
            object.id as i64
        )
        .execute(&self.db)
        .await
        .map_err(|err| Status::new(Aborted, err.to_string()))?;
        Ok(Response::new(object))
    }

    async fn delete_object(&self, request: Request<ObjectId>) -> Result<Response<()>, Status> {
        let object_id = request.into_inner().id;
        query!(r#"DELETE FROM object WHERE id = $1"#, object_id as i64)
            .execute(&self.db)
            .await
            .map_err(|err| Status::new(Aborted, err.to_string()))?;
        Ok(Response::new(()))
    }

    type subscribe_objectStream = ResponseStream<Object>;

    async fn subscribe_object(
        &self,
        request: Request<ObjectId>,
    ) -> Result<Response<Self::subscribe_objectStream>, Status> {
        todo!()
    }
}

// Rest Routes
pub fn object_route() -> Router<SharedState> {
    Router::new()
        .route("/objects", get(list_objects))
        .route("/object/:id", get(get_object))
        .route("/object", post(create_object))
        .route("/object/:id", put(update_object))
        .route("/object/:id", delete(delete_object))
        .layer(TraceLayer::new_for_http()) // logging with tower-http middleware
}

async fn list_objects(State(state): State<SharedState>) -> impl IntoResponse {
    let grpc_request = Request::new(());
    let grpc_response = state.object_service.list_objects(grpc_request).await;
    grpc_response
        .map(|res| AxumResponse::from(res))
        .map_err(|status| status.to_string())
}

async fn get_object(State(state): State<Arc<AppState>>, Path(id): Path<u64>) -> impl IntoResponse {
    let grpc_request = Request::new(ObjectId { id });
    let grpc_response = state.object_service.get_object(grpc_request).await;
    grpc_response
        .map(|res| AxumResponse::from(res))
        .map_err(|status| status.to_string())
}

async fn create_object(
    State(state): State<SharedState>,
    Json(input): Json<Object>,
) -> impl IntoResponse {
    let grpc_request = Request::new(input);
    let grpc_response = state.object_service.create_object(grpc_request).await;
    grpc_response
        .map(|res| AxumResponse::from(res))
        .map_err(|status| status.to_string())
}

async fn update_object(
    State(state): State<SharedState>,
    Json(input): Json<Object>,
) -> impl IntoResponse {
    let grpc_request = Request::new(input);
    let grpc_response = state.object_service.update_object(grpc_request).await;
    grpc_response
        .map(|res| AxumResponse::from(res))
        .map_err(|status| status.to_string())
}

async fn delete_object(State(state): State<SharedState>, Path(id): Path<u64>) -> impl IntoResponse {
    let grpc_request = Request::new(ObjectId { id });
    let grpc_response = state.object_service.delete_object(grpc_request).await;
    grpc_response
        .map(|res| AxumResponse::from(res))
        .map_err(|status| status.to_string())
}
