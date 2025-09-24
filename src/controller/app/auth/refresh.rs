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
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde_json::json;
use tower_cookies::Cookies;

#[tracing::instrument(skip_all)]
pub async fn get(
    State(state): State<SharedAppState>,
    cookies: Cookies,
) -> Result<Response, GenericAuthError> {
    let refresh_cookie =
        cookies
            .get(auth::REFRESH_COOKIE)
            .ok_or(GenericAuthError::GenericAuthError(
                AuthError::MissingCredentials,
            ))?;
    let mut ctx = Ctx::from_jwt(
        refresh_cookie.value(),
        &DecodingKey::from_secret(state.auth_secret.as_bytes()),
    )?;
    let auth_jwt = ctx
        .as_auth()
        .to_jwt(EncodingKey::from_secret(state.auth_secret.as_bytes()))?;
    dbg!(&auth_jwt);
    let mut headers = HeaderMap::new();
    headers.append(AUTH_HEADER, auth_jwt.parse().expect("can't parse auth"));
    Ok((headers, json!({"jwt":auth_jwt,"ctx":ctx}).to_string()).into_response())
}
