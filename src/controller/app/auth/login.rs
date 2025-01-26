use jsonwebtoken::EncodingKey;
use tower_cookies::cookie::time::Duration;

use crate::{
    app_state::SharedAppState,
    auth::{
        mw_auth::{CtxResult, AUTH_HEADER, REFRESH_COOKIE},
        password::{validate_credentials, Credentials},
    },
    controller::error::GenericAuthError,
};
use axum::{
    extract::{Extension, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Form,
};
use tower_cookies::{Cookie, Cookies};

#[axum::debug_handler]
pub async fn post(
    State(state): State<SharedAppState>,
    Extension(ctx_res): Extension<CtxResult>,
    cookies: Cookies,
    Form(credentials): Form<Credentials>,
) -> Result<Response, GenericAuthError> {
    if ctx_res.is_ok() {
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
