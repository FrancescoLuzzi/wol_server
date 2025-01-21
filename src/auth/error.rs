#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum CtxError {
    #[error("Can't decode credentials from jwt: {0}")]
    JwtDecodeError(#[source] jsonwebtoken::errors::Error),
    #[error("Can't decode credentials from jwt: {0}")]
    JwtEncodeError(#[source] jsonwebtoken::errors::Error),
}
