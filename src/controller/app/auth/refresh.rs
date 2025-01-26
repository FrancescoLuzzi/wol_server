use crate::{
    app_state::SharedAppState,
    auth::{ctx::Ctx, mw_auth::AUTH_HEADER},
    controller::error::GenericAuthError,
};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Extension, IntoResponse, Response},
};
use jsonwebtoken::EncodingKey;
use serde_json::json;

// guard this with mw_ctx_require
pub async fn post(
    State(state): State<SharedAppState>,
    Extension(mut ctx): Extension<Ctx>,
) -> Result<Response, GenericAuthError> {
    let ctx = ctx.as_auth();
    let auth_jwt = ctx.to_jwt(EncodingKey::from_secret(state.hmac_secret.as_bytes()))?;
    let mut headers = HeaderMap::new();
    headers.append(AUTH_HEADER, auth_jwt.parse().expect("can't parse auth"));
    Ok((headers, json!({"jwt":auth_jwt,"ctx":ctx}).to_string()).into_response())
}
