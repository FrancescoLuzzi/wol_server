use axum::{http::StatusCode, response::IntoResponse};

use crate::auth::error::{AuthError, CtxError};

#[derive(thiserror::Error, Debug)]
pub enum GenericAuthError {
    #[error(transparent)]
    GenericCtxError(#[from] CtxError),
    #[error(transparent)]
    GenericAuthError(#[from] AuthError),
    #[error("Unkwown error")]
    GenericUnknownError(#[from] anyhow::Error),
}

impl IntoResponse for GenericAuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GenericAuthError::GenericCtxError(ctx_error) => ctx_error.into_response(),
            GenericAuthError::GenericAuthError(auth_error) => auth_error.into_response(),
            GenericAuthError::GenericUnknownError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unknown error: {}", error.source().unwrap()),
            )
                .into_response(),
        }
    }
}

#[derive(Debug)]
pub struct UnknownError(anyhow::Error);

impl IntoResponse for UnknownError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unknown error: {}", self.0),
        )
            .into_response()
    }
}
impl<E> From<E> for UnknownError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
