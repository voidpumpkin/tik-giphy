use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulResponseBody<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponseBody<T> {
    pub errors: T,
}
