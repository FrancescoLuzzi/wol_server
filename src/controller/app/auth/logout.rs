use crate::auth::logout;
use axum::http::StatusCode;
use tower_cookies::Cookies;

#[tracing::instrument(skip_all)]
pub async fn post(cookies: Cookies) -> Result<StatusCode, StatusCode> {
    match logout::logout(cookies).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
