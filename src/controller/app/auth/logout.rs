use crate::auth::{ctx::Ctx, logout};
use axum::http::StatusCode;
use tower_cookies::Cookies;

#[tracing::instrument(skip_all)]
pub async fn post(ctx: Ctx, cookies: Cookies) -> Result<StatusCode, StatusCode> {
    match logout::logout(ctx, cookies).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
