use core::fmt;

use ntex::{
    http::StatusCode,
    web::{HttpResponse, WebResponseError},
};

#[derive(Debug, Clone)]
pub enum WebError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
}

impl WebResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::NotFound(_) => StatusCode::NOT_FOUND,
            WebError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self, _: &ntex::web::HttpRequest) -> HttpResponse {
        HttpResponse::new(self.status_code()).set_body(
            match self {
                WebError::NotFound(e) => e,
                WebError::InternalServerError(e) => e,
                WebError::BadRequest(e) => e,
            }
            .into(),
        )
    }
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebError::NotFound(e) => write!(f, "{e}"),
            WebError::InternalServerError(e) => write!(f, "{e}"),
            WebError::BadRequest(e) => write!(f, "{e}"),
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
