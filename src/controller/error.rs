use axum::response::IntoResponse;

use crate::auth::error::{AuthError, CtxError};

#[derive(thiserror::Error, Debug)]
pub enum GenericAuthError {
    #[error(transparent)]
    GenericCtxError(#[from] CtxError),
    #[error(transparent)]
    GenericAuthError(#[from] AuthError),
}

impl IntoResponse for GenericAuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GenericAuthError::GenericCtxError(ctx_error) => ctx_error.into_response(),
            GenericAuthError::GenericAuthError(auth_error) => auth_error.into_response(),
        }
    }
}
