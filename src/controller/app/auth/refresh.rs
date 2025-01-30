use crate::{
    app_state::SharedAppState,
    auth::{self, ctx::Ctx, error::AuthError, AUTH_HEADER},
    controller::error::GenericAuthError,
};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use axum_extra::headers::Cookie;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde_json::json;

pub async fn post(
    cookies: Cookie,
    State(state): State<SharedAppState>,
) -> Result<Response, GenericAuthError> {
    let refresh_cookie =
        cookies
            .get(auth::REFRESH_COOKIE)
            .ok_or(GenericAuthError::GenericAuthError(
                AuthError::MissingCredentials,
            ))?;
    let ctx = Ctx::from_jwt(
        refresh_cookie,
        &DecodingKey::from_secret(state.hmac_secret.as_bytes()),
    )?;
    let auth_jwt = ctx.to_jwt(EncodingKey::from_secret(state.hmac_secret.as_bytes()))?;
    let mut headers = HeaderMap::new();
    headers.append(AUTH_HEADER, auth_jwt.parse().expect("can't parse auth"));
    Ok((headers, json!({"jwt":auth_jwt,"ctx":ctx}).to_string()).into_response())
}
