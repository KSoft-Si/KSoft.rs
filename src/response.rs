use crate::error::api::ResponseError;

pub enum ApiResponse<T> {
    Success(T),
    Failed(ResponseError)
}