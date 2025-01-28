use super::ctx::Ctx;
use crate::app_state::SharedAuthState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use jsonwebtoken::decode;
use jsonwebtoken::{DecodingKey, Validation};
use tower_cookies::{Cookie, Cookies};

pub type CtxResult = Result<Ctx, CtxExtError>;

pub const AUTH_HEADER: &str = "Authorization";
pub const REFRESH_COOKIE: &str = "WOL_REFRESH_TOKEN";

pub async fn mw_ctx_require_admin(
    Extension(ctx): Extension<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, CtxExtError> {
    dbg!("{:<12} - mw_ctx_require_admin - {ctx:?}", "MIDDLEWARE");

    if !ctx.is_admin() {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    Ok(next.run(req).await)
}

pub async fn mw_ctx_require_totp(
    Extension(ctx): Extension<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, CtxExtError> {
    dbg!("{:<12} - mw_ctx_require_totp - {ctx:?}", "MIDDLEWARE");

    if !ctx.valid_totp {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    Ok(next.run(req).await)
}

pub async fn mw_ctx_require(
    Extension(ctx_res): Extension<CtxResult>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, CtxExtError> {
    dbg!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    match ctx_res {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => Ok(StatusCode::UNAUTHORIZED.into_response()),
    }
}

pub async fn mw_ctx_resolver(
    State(state): State<SharedAuthState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    dbg!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = ctx_resolve(state, req.headers()).await;
    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        req.headers_mut().remove(AUTH_HEADER);
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(state: SharedAuthState, headers: &HeaderMap<HeaderValue>) -> CtxResult {
    let auth_token = headers
        .get(AUTH_HEADER)
        .ok_or(CtxExtError::TokenNotInCookie)?;
    let mut token_parts = auth_token
        .to_str()
        .map_err(|_| CtxExtError::TokenMalformed)?
        .split(" ");
    if token_parts.next() != Some("Bearer") {
        return Err(CtxExtError::TokenMalformed);
    }

    decode::<Ctx>(
        token_parts.next().ok_or(CtxExtError::TokenMalformed)?,
        &DecodingKey::from_secret(state.hmac_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|x| x.claims)
    .map_err(|_| CtxExtError::TokenMalformed)
}

#[derive(Clone, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenMalformed,
    SessionNotFound,
    SessionAccessError,
    CannotSetTokenCookie,

    CtxNotInRequestExt,
    CtxCreateFail(String),
}
impl IntoResponse for CtxExtError {
    fn into_response(self) -> Response {
        tracing::error!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
