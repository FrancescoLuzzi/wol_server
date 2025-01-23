use crate::auth::{logout, mw_auth::CtxResult};
use axum::{http::StatusCode, response::Extension};
use tower_cookies::Cookies;

pub async fn post(
    Extension(ctx_res): Extension<CtxResult>,
    cookies: Cookies,
) -> Result<StatusCode, StatusCode> {
    if ctx_res.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    match logout::logout(ctx_res.unwrap(), cookies).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
