use crate::{
    auth::{ctx::Ctx, mw_auth::AUTH_HEADER},
    controller::error::GenericAuthError,
};
use axum::{
    http::HeaderMap,
    response::{Extension, IntoResponse, Redirect, Response},
};
use jsonwebtoken::EncodingKey;
use serde_json::json;

pub async fn get(Extension(mut ctx): Extension<Ctx>) -> Result<Response, GenericAuthError> {
    if let Some(valid_totp) = ctx.valid_totp {
        if !valid_totp {
            return Ok(Redirect::to("/auth/totp").into_response());
        }
        let ctx = ctx.as_auth();
        let auth_jwt = ctx.to_jwt(EncodingKey::from_secret(b"ciccio"))?;
        let mut headers = HeaderMap::new();
        headers.append(AUTH_HEADER, auth_jwt.parse().expect("can't parse auth"));
        Ok((headers, json!({"jwt":auth_jwt,"ctx":ctx}).to_string()).into_response())
    } else {
        Ok(Redirect::to("/auth/totp").into_response())
    }
}

pub async fn post(Extension(mut ctx): Extension<Ctx>) -> Result<Response, GenericAuthError> {
    if let Some(valid_totp) = ctx.valid_totp {
        if !valid_totp {
            return Ok(Redirect::to("/auth/totp").into_response());
        }
        let ctx = ctx.as_auth();
        let auth_jwt = ctx.to_jwt(EncodingKey::from_secret(b"ciccio"))?;
        let mut headers = HeaderMap::new();
        headers.append(AUTH_HEADER, auth_jwt.parse().expect("can't parse auth"));
        Ok((headers, json!({"jwt":auth_jwt,"ctx":ctx}).to_string()).into_response())
    } else {
        Ok(Redirect::to("/auth/totp").into_response())
    }
}
