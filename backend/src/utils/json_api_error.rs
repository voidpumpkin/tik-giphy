use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonApiError {
    pub title: String,
}
