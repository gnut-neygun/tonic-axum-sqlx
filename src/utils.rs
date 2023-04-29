use std::pin::Pin;

use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use tonic::codegen::futures_core::Stream;
use tonic::Status;

pub struct AxumResponse<T> {
    response: tonic::Response<T>,
}

impl<T> IntoResponse for AxumResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let value = Json(self.response.into_inner());
        value.into_response()
    }
}

impl<T> From<tonic::Response<T>> for AxumResponse<T> {
    fn from(value: tonic::Response<T>) -> Self {
        AxumResponse { response: value }
    }
}

pub type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;
