use std::pin::Pin;

use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tonic::codegen::futures_core::Stream;
use tonic::Status;

pub struct TonicResponse<T> {
    response: tonic::Response<T>,
}

impl<T> IntoResponse for TonicResponse<T> where T: Serialize {
    fn into_response(self) -> Response {
        let value = Json(self.response.into_inner());
        value.into_response()
    }
}

impl<T> From<tonic::Response<T>> for TonicResponse<T> {
    fn from(value: tonic::Response<T>) -> Self {
        TonicResponse {
            response: value
        }
    }
}


pub type ResponseStream<T> = Pin<Box<dyn Stream<Item=Result<T, Status>> + Send>>;