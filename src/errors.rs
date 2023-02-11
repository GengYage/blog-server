use core::fmt;

use ntex::{
    http::StatusCode,
    web::{HttpResponse, WebResponseError},
};

#[derive(Debug, Clone)]
pub enum WebError {
    NotFound(String),
    InternalServerError(String),
}

impl WebResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::NotFound(_) => StatusCode::NOT_FOUND,
            WebError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &ntex::web::HttpRequest) -> HttpResponse {
        HttpResponse::new(self.status_code()).set_body(
            match self {
                WebError::NotFound(e) => e,
                WebError::InternalServerError(e) => e,
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
        }
    }
}
