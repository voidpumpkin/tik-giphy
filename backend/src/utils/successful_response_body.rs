use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulResponseBody<T> {
    pub data: T,
}
