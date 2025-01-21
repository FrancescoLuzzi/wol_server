use super::ctx::Ctx;
use crate::app_state::SharedAuthState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use jsonwebtoken::decode;
use jsonwebtoken::{DecodingKey, Validation};
use secrecy::ExposeSecret;
use tower_cookies::{Cookie, Cookies};

pub type CtxResult = Result<Ctx, CtxExtError>;
pub const AUTH_COOKIE: &str = "WOL_AUTH_TOKEN";
pub const REFRESH_COOKIE: &str = "WOL_REFRESh_TOKEN";

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
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    dbg!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = ctx_resolve(state, &cookies).await;
    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_COOKIE))
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(state: SharedAuthState, cookies: &Cookies) -> CtxResult {
    let auth_token = cookies
        .get(AUTH_COOKIE)
        .ok_or(CtxExtError::TokenNotInCookie)?;
    decode::<Ctx>(
        auth_token.value(),
        &DecodingKey::from_secret(state.hmac_secret.expose_secret().as_bytes()),
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
