use axum::{
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use super::ApiHttpResponse;

impl IntoResponse for ApiHttpResponse {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let status = StatusCode::from_u16(self.code)
            .unwrap_or_default()
            .to_string();

        println!("Status: {:?}", status);

        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());

        let body = json!({
            "code": self.code,
            "message": self.message,
            "data": self.data,
            "status": status,
            "errors": self.errors
        });

        (status_code, headers, Json(body)).into_response()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_response() {
        let response = ApiHttpResponse::default();

        let default_response = ApiHttpResponse {
            code: 200,
            status: String::from("success"),
            message: String::new(),
            data: Some(json!("[]")),
            errors: Some(json!("[]")),
        };

        assert_eq!(response, default_response)
    }
}
