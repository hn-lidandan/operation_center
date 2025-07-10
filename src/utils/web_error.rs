use actix_web::{HttpResponse, Result, error, http::StatusCode};
use serde::Serialize;
use serde_json;
use sqlx::error::Error as SQLxError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum WebError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct WebErrorResponse {
    error_message: String,
}

impl WebError {
    fn error_response(&self) -> String {
        match self {
            WebError::DBError(msg) => msg.into(),
            WebError::ActixError(msg) => {
                // "Internal server error".into()
                msg.into()
            }
            WebError::NotFound(msg) => msg.into(),
        }
    }
}
//实现 error::ResponseError 接口
impl error::ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::DBError(_msg) | WebError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            WebError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(WebErrorResponse {
            error_message: self.error_response(),
        })
    }
}
// 实现 Display 用于用户友好输出
impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.error_response())
    }
}
// 使actix_web::error::Error能转化为WebError
impl From<actix_web::error::Error> for WebError {
    fn from(err: actix_web::error::Error) -> Self {
        WebError::ActixError(err.to_string())
    }
}
//使 SQLxError能转化为WebError
impl From<SQLxError> for WebError {
    fn from(err: SQLxError) -> Self {
        WebError::DBError(err.to_string())
    }
}

impl From<sea_orm::DbErr> for WebError {
    fn from(err: sea_orm::DbErr) -> Self {
        WebError::DBError(err.to_string())
    }
}

impl From<serde_json::Error> for WebError {
    fn from(err: serde_json::Error) -> Self {
        WebError::ActixError(err.to_string())
    }
}

impl From<std::io::Error> for WebError {
    fn from(err: std::io::Error) -> Self {
        WebError::ActixError(err.to_string())
    }
}

impl StdError for WebError {}
