//! What can go wrong outside the kernel. The kernel's own refusals are outcomes, not errors.

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not a member of this cohort")]
    NotMember,
    #[error("the cohort's lesson {0:?} is not loaded")]
    UnknownLesson(String),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error("stored data does not match the kernel's types: {0}")]
    Corrupt(#[from] serde_json::Error),
}
