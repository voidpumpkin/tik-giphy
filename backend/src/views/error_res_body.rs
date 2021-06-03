use crate::constants::TITLE;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResBody {
    pub errors: Vec<serde_json::Value>,
}

impl ErrorResBody {
    pub fn new_single_error(err: &str) -> ErrorResBody {
        ErrorResBody {
            errors: vec![json!({ TITLE: err.to_string() })],
        }
    }
}

impl From<ValidationErrors> for ErrorResBody {
    fn from(validation_errors: ValidationErrors) -> Self {
        Self {
            errors: validation_errors
                .field_errors()
                .iter()
                .map(|field_err| {
                    field_err.1.iter().map(|validation_err| {
                        let title = match validation_err.clone().message {
                            Some(message) => message.to_string(),
                            None => format!("{}", validation_err),
                        };
                        json!({ TITLE: title })
                    })
                })
                .flatten()
                .collect(),
        }
    }
}
