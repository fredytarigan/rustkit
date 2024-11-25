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

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum ApiStatusVariant {
    ResponseOk,
    ResponseFailed(FailedResponseCode),
    ResponseError(ErrorResponseCode),
}

/*
    FailedResponseCode is collection of 4xx response code
    * BadRequest            400
    * Unauthorizde          401
    * Forbidden             403
    * NotFound              404
    * MethodNotAllowed      405
    * NotAcceptable         406
    * RequestTimeout        408
    * Conflict               409
    * UnsupportedMediaType  415
    * UnprocessableContent  422
    * TooManyRequest        429
*/
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum FailedResponseCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    RequestTimeout,
    Conflict,
    UnsupportedMediaType,
    UnprocessableContent,
    TooManyRequest,
}

/*
    ErrorResponseCode is collection fo 5xx response code
    * InternalServerError   500
    * NotImplemented        501
    * BadGateway            502
    * ServiceUnavailable    503
    * GatewayTimeout        504
    * UnknwonError          *
*/
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorResponseCode {
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    UnknownError,
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

impl ToString for ApiStatusVariant {
    fn to_string(&self) -> String {
        match self {
            ApiStatusVariant::ResponseOk => String::from("success"),
            ApiStatusVariant::ResponseFailed(e) => match e {
                FailedResponseCode::BadRequest => String::from("bad request"),
                FailedResponseCode::Unauthorized => String::from("unauthorized"),
                FailedResponseCode::Forbidden => String::from("forbidden"),
                FailedResponseCode::NotFound => String::from("not found"),
                FailedResponseCode::MethodNotAllowed => String::from("method not allowed"),
                FailedResponseCode::NotAcceptable => String::from("not acceptable"),
                FailedResponseCode::RequestTimeout => String::from("request timeout"),
                FailedResponseCode::Conflict => String::from("conflict"),
                FailedResponseCode::UnsupportedMediaType => String::from("unsupported media type"),
                FailedResponseCode::UnprocessableContent => String::from("unprocessable content"),
                FailedResponseCode::TooManyRequest => String::from("too many request"),
            },
            ApiStatusVariant::ResponseError(e) => match e {
                ErrorResponseCode::InternalServerError => String::from("internal server error"),
                ErrorResponseCode::NotImplemented => String::from("not implemented"),
                ErrorResponseCode::BadGateway => String::from("bad gateway"),
                ErrorResponseCode::ServiceUnavailable => String::from("service unavailable"),
                ErrorResponseCode::GatewayTimeout => String::from("gateway timeout"),
                ErrorResponseCode::UnknownError => String::from("unknown error"),
            },
        }
    }
}

impl TryFrom<u16> for ApiStatusVariant {
    type Error = ();

    fn try_from(status: u16) -> Result<Self, Self::Error> {
        match status {
            /* 2xx error code */
            200 => Ok(ApiStatusVariant::ResponseOk),

            /* 4xx error code */
            400 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::BadRequest,
            )),
            401 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::Unauthorized,
            )),
            403 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::Forbidden,
            )),
            404 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::NotFound,
            )),
            405 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::MethodNotAllowed,
            )),
            406 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::NotAcceptable,
            )),
            408 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::RequestTimeout,
            )),
            409 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::Conflict,
            )),
            415 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::UnsupportedMediaType,
            )),
            422 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::UnprocessableContent,
            )),
            429 => Ok(ApiStatusVariant::ResponseFailed(
                FailedResponseCode::TooManyRequest,
            )),

            /* 5xx error code */
            500 => Ok(ApiStatusVariant::ResponseError(
                ErrorResponseCode::InternalServerError,
            )),
            501 => Ok(ApiStatusVariant::ResponseError(
                ErrorResponseCode::NotImplemented,
            )),
            502 => Ok(ApiStatusVariant::ResponseError(
                ErrorResponseCode::BadGateway,
            )),
            503 => Ok(ApiStatusVariant::ResponseError(
                ErrorResponseCode::ServiceUnavailable,
            )),
            504 => Ok(ApiStatusVariant::ResponseError(
                ErrorResponseCode::GatewayTimeout,
            )),

            _ => Ok(ApiStatusVariant::ResponseError(
                ErrorResponseCode::UnknownError,
            )),
        }
    }
}
