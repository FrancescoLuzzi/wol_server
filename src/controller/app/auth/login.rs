use crate::{
    app_state::SharedAppState,
    auth::{
        ctx::Ctx,
        password::{validate_credentials, Credentials},
        AUTH_HEADER, REFRESH_COOKIE,
    },
    controller::error::GenericAuthError,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Form,
};
use jsonwebtoken::EncodingKey;
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};

#[axum::debug_handler]
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
    tracing::Span::current().record("user_id", &tracing::field::display(&user_ctx.user_id));
    let auth_jwt = user_ctx
        .as_auth()
        .to_jwt(EncodingKey::from_secret(state.hmac_secret.as_bytes()))?;
    let refresh_jwt = user_ctx
        .as_refresh()
        .to_jwt(EncodingKey::from_secret(state.hmac_secret.as_bytes()))?;
    let refresh_cookie = Cookie::build((REFRESH_COOKIE, refresh_jwt))
        .max_age(Duration::days(30))
        .http_only(true)
        .build();
    cookies.add(refresh_cookie);
    let mut headers = HeaderMap::new();
    headers.append(AUTH_HEADER, auth_jwt.parse().expect("can't parse auth"));
    Ok((headers, StatusCode::OK).into_response())
}
