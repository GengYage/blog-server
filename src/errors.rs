use core::fmt;

use ntex::{
    http::{client::error::SendRequestError, StatusCode},
    web::{HttpResponse, WebResponseError},
};

#[derive(Debug, Clone)]
pub enum WebError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    AuthFailed(String),
}

impl WebResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::NotFound(_) => StatusCode::BAD_REQUEST,
            WebError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::BadRequest(_) => StatusCode::BAD_REQUEST,
            WebError::AuthFailed(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self, _: &ntex::web::HttpRequest) -> HttpResponse {
        HttpResponse::new(self.status_code()).set_body({
            let error_msg = match self {
                WebError::NotFound(e) => e,
                WebError::InternalServerError(e) => e,
                WebError::BadRequest(e) => e,
                WebError::AuthFailed(e) => e,
            };
            format!(r#"{{"error":"{}"}}"#, error_msg).into()
        })
    }
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebError::NotFound(e) => write!(f, r#"{{"error":"{e}"}}"#),
            WebError::InternalServerError(e) => write!(f, r#"{{"error":"{e}"}}"#),
            WebError::BadRequest(e) => write!(f, r#"{{"error":"{e}"}}"#),
            WebError::AuthFailed(e) => write!(f, r#"{{"error":"{e}"}}"#),
        }
    }
}

impl From<sqlx::Error> for WebError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound("找不到对应的数据".into()),
            _ => Self::InternalServerError("服务器内部错误".into()),
        }
    }
}

impl From<SendRequestError> for WebError {
    fn from(value: SendRequestError) -> Self {
        match value {
            SendRequestError::Timeout => Self::InternalServerError("rest github timeout".into()),
            SendRequestError::Url(e) => Self::InternalServerError(e.to_string()),
            SendRequestError::Connect(e) => Self::InternalServerError(e.to_string()),
            SendRequestError::Send(e) => Self::InternalServerError(e.to_string()),
            SendRequestError::Response(e) => Self::InternalServerError(e.to_string()),
            SendRequestError::Http(e) => Self::InternalServerError(e.to_string()),
            SendRequestError::H2(e) => Self::InternalServerError(e.to_string()),
            SendRequestError::TunnelNotSupported => {
                Self::InternalServerError("tunnel not supported".into())
            }
            SendRequestError::Error(e) => Self::InternalServerError(e.to_string()),
        }
    }
}
