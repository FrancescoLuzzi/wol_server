use crate::{
    app_state::SharedAppState,
    auth::{ctx::Ctx, AUTH_HEADER, REFRESH_COOKIE},
    controller::error::GenericAuthError,
};
use anyhow::Context;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use jsonwebtoken::EncodingKey;
use rand::Rng as _;
use serde_json::json;
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};

pub async fn get_regenerate(
    State(state): State<SharedAppState>,
    cookies: Cookies,
    mut ctx: Ctx,
) -> Result<Response, GenericAuthError> {
    let ctx = ctx.as_auth();
    let mut transaction = state
        .db_pool
        .begin()
        .await
        .context("can't start transaction")?;
    let totp_secret = rand::thread_rng().gen::<[u8; 21]>().to_vec();
    sqlx::query!(
        r#"UPDATE users
    SET totp_secret=$1,
        update_date=datetime('now','localtime')
    WHERE
        id=$2"#,
        totp_secret,
        ctx.user_id,
    )
    .execute(&mut *transaction)
    .await
    .context("can't update totp")?;
    let refresh_jwt =
        ctx.as_refresh()
            .with_valid_totp(false)
            .to_jwt(jsonwebtoken::EncodingKey::from_secret(
                state.hmac_secret.as_bytes(),
            ))?;
    let refresh_cookie = Cookie::build((REFRESH_COOKIE, refresh_jwt))
        .max_age(Duration::days(30))
        .http_only(true)
        .build();
    cookies.add(refresh_cookie);
    let totp_url = totp_rs::TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        totp_secret,
        Some(state.base_url.clone()),
        "".to_string(),
    )
    .context("error creating totp url")?
    .get_url();
    transaction
        .commit()
        .await
        .context("can't commit transaction")?;
    Ok(totp_url.into_response())
}

pub async fn post(
    State(state): State<SharedAppState>,
    mut ctx: Ctx,
) -> Result<Response, GenericAuthError> {
    let ctx = ctx.as_auth();
    let auth_jwt = ctx.to_jwt(EncodingKey::from_secret(state.hmac_secret.as_bytes()))?;
    let mut headers = HeaderMap::new();
    headers.append(AUTH_HEADER, auth_jwt.parse().context("can't parse auth")?);
    Ok((headers, json!({"jwt":auth_jwt,"ctx":ctx}).to_string()).into_response())
}
