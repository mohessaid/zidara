//! What can go wrong outside the kernel. The kernel's refusals of commands are outcomes, not
//! errors; a kernel error here means the server is misconfigured (an unknown framework).

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use zidara_kernel::decide::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("sign in first")]
    Unauthorized,
    #[error("not a member of this cohort")]
    NotMember,
    #[error("your role cannot see this")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("the cohort's lesson {0:?} is not loaded")]
    UnknownLesson(String),
    #[error(transparent)]
    Kernel(#[from] KernelError),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error("stored data does not match the kernel's types: {0}")]
    Corrupt(#[from] serde_json::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotMember | ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::UnknownLesson(_)
            | ApiError::Kernel(_)
            | ApiError::Database(_)
            | ApiError::Corrupt(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}
