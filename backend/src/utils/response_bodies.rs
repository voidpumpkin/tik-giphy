use crate::utils::JsonApiError;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulResBody<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResBody<T> {
    pub errors: Vec<T>,
}

impl ErrorResBody<JsonApiError> {
    pub fn new_single_error(err: &str) -> ErrorResBody<JsonApiError> {
        ErrorResBody {
            errors: vec![JsonApiError {
                title: err.to_string(),
            }],
        }
    }
}
