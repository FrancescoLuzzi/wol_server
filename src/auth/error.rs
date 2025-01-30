use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Missing credentials.")]
    MissingCredentials,
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error("Invalid jwt.")]
    CtxError(#[from] CtxError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::InvalidCredentials(_) | AuthError::CtxError(_) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
            AuthError::MissingCredentials => StatusCode::FORBIDDEN.into_response(),
            AuthError::UnexpectedError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("something went wrong {}", error),
            )
                .into_response(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CtxError {
    #[error("Can't decode credentials from jwt: {0}")]
    JwtDecodeError(#[source] jsonwebtoken::errors::Error),
    #[error("Can't decode credentials from jwt: {0}")]
    JwtEncodeError(#[source] jsonwebtoken::errors::Error),
}

impl IntoResponse for CtxError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error with jwt: {}", self.to_string()),
        )
            .into_response()
    }
}
