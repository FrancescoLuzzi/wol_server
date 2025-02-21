use crate::{
    app_state::SharedAppState,
    auth::{
        ctx::Ctx,
        password::{validate_credentials, Credentials},
        REFRESH_COOKIE,
    },
    controller::error::GenericAuthError,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use jsonwebtoken::EncodingKey;
use serde_json::json;
use tower_cookies::cookie::{time::Duration, SameSite};
use tower_cookies::{Cookie, Cookies};

#[tracing::instrument(skip_all)]
pub async fn post(
    State(state): State<SharedAppState>,
    ctx: Option<Ctx>,
    cookies: Cookies,
    Form(credentials): Form<Credentials>,
) -> Result<Response, GenericAuthError> {
    if ctx.is_some() {
        return Ok(StatusCode::OK.into_response());
    }
    let mut user_ctx = validate_credentials(credentials, &state.db_pool).await?;
    tracing::Span::current().record("user_id", tracing::field::display(&user_ctx.user_id));
    let auth_jwt = user_ctx
        .as_auth()
        .to_jwt(EncodingKey::from_secret(state.hmac_secret.as_bytes()))?;
    let refresh_jwt = user_ctx
        .as_refresh()
        .to_jwt(EncodingKey::from_secret(state.hmac_secret.as_bytes()))?;
    let refresh_cookie = Cookie::build((REFRESH_COOKIE, refresh_jwt))
        .max_age(Duration::days(30))
        .same_site(SameSite::Lax)
        .path("/")
        .http_only(true)
        .build();
    cookies.add(refresh_cookie);
    Ok((json!({"jwt":auth_jwt,"ctx":ctx}).to_string()).into_response())
}
