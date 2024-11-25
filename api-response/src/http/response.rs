use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ApiHttpResponse {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<Value>,
    pub errors: Option<Value>,
}

impl Default for ApiHttpResponse {
    fn default() -> Self {
        Self {
            code: 200,
            status: String::from("success"),
            message: String::new(),
            data: Some(json!("[]")),
            errors: Some(json!("[]")),
        }
    }
}
