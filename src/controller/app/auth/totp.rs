use crate::{
    app_state::SharedAppState,
    auth::{ctx::Ctx, error::AuthError, REFRESH_COOKIE},
    controller::error::GenericAuthError,
};
use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::Rng as _;
use serde_json::json;
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};

#[derive(serde::Deserialize)]
struct TotpSecret {
    totp_secret: Vec<u8>,
}

#[derive(serde::Deserialize)]
pub struct TotpRequest {
    totp: String,
}

#[tracing::instrument(skip_all)]
pub async fn get_regenerate(
    State(state): State<SharedAppState>,
    cookies: Cookies,
    mut ctx: Ctx,
) -> Result<Response, GenericAuthError> {
    let mut transaction = state
        .db_pool
        .begin()
        .await
        .context("can't start transaction")?;
    let totp_secret = rand::thread_rng().gen::<[u8; 21]>().to_vec();
    sqlx::query!(
        r#"INSERT INTO totp_request(user_id,totp_secret)
            VALUES($1,$2)
        ON CONFLICT(user_id) DO UPDATE SET totp_secret=$2"#,
        ctx.user_id,
        totp_secret,
    )
    .execute(&mut *transaction)
    .await
    .context("can't update totp")?;
    let refresh_jwt =
        ctx.as_refresh()
            .with_valid_totp(false)
            .to_jwt(jsonwebtoken::EncodingKey::from_secret(
                state.auth_secret.as_bytes(),
            ))?;
    let refresh_cookie = Cookie::build((REFRESH_COOKIE, refresh_jwt))
        .max_age(Duration::days(30))
        .http_only(true)
        .path("/")
        .build();
    cookies.add(refresh_cookie);
    let totp = totp_rs::TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        totp_secret,
        Some(state.app_name.clone()),
        state.app_name.clone(),
    )
    .context("error creating totp url")?;
    let totp_url = totp.get_url();
    let totp_secret = totp.get_secret_base32();
    transaction
        .commit()
        .await
        .context("can't commit transaction")?;
    Ok(json!({"url":totp_url,"secret":totp_secret})
        .to_string()
        .into_response())
}

#[tracing::instrument(skip_all)]
pub async fn post_validate(
    State(state): State<SharedAppState>,
    mut ctx: Ctx,
    cookies: Cookies,
    Json(totp): Json<TotpRequest>,
) -> Result<Response, GenericAuthError> {
    let mut transaction = state
        .db_pool
        .begin()
        .await
        .context("can't start transaction")?;
    let totp_secret = sqlx::query_as!(
        TotpSecret,
        r#"DELETE FROM totp_request
           WHERE user_id=$1
        RETURNING totp_secret"#,
        ctx.user_id,
    )
    .fetch_optional(&mut *transaction)
    .await
    .context("can't fetch totp")?;
    match totp_secret {
        Some(totp_secret) => {
            let is_valid = totp_rs::TOTP::new(
                totp_rs::Algorithm::SHA1,
                6,
                1,
                30,
                totp_secret.totp_secret.clone(),
                Some(state.app_name.clone()),
                state.app_name.clone(),
            )
            .context("error creating totp url")?
            .check(&totp.totp, Utc::now().timestamp() as u64);
            match is_valid {
                true => {
                    let refresh_jwt = ctx.as_refresh().with_valid_totp(true).to_jwt(
                        jsonwebtoken::EncodingKey::from_secret(state.auth_secret.as_bytes()),
                    )?;
                    let auth_token =
                        ctx.as_auth()
                            .to_jwt(jsonwebtoken::EncodingKey::from_secret(
                                state.auth_secret.as_bytes(),
                            ))?;
                    sqlx::query!(
                        r#"UPDATE users
                        SET totp_secret=$1
                        WHERE id=$2"#,
                        totp_secret.totp_secret,
                        ctx.user_id,
                    )
                    .execute(&mut *transaction)
                    .await
                    .context("can't update totp")?;
                    transaction
                        .commit()
                        .await
                        .context("can't commit transaction")?;
                    let refresh_cookie = Cookie::build((REFRESH_COOKIE, refresh_jwt))
                        .max_age(Duration::days(30))
                        .http_only(true)
                        .path("/")
                        .build();
                    cookies.add(refresh_cookie);
                    Ok(auth_token.into_response())
                }
                false => return Ok(StatusCode::BAD_REQUEST.into_response()),
            }
        }
        None => Ok(StatusCode::BAD_REQUEST.into_response()),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post(
    State(state): State<SharedAppState>,
    cookies: Cookies,
    Json(secret): Json<TotpRequest>,
) -> Result<Response, GenericAuthError> {
    let mut refresh_cookie =
        cookies
            .get(REFRESH_COOKIE)
            .ok_or(GenericAuthError::GenericAuthError(
                AuthError::MissingCredentials,
            ))?;
    let mut ctx = Ctx::from_jwt(
        refresh_cookie.value(),
        &DecodingKey::from_secret(state.auth_secret.as_bytes()),
    )?;

    let totp_secret = sqlx::query_as!(
        TotpSecret,
        r#"SELECT totp_secret FROM  users
        WHERE
            id=$1"#,
        ctx.user_id,
    )
    .fetch_one(&state.db_pool)
    .await
    .context("can't get totp")?;

    match totp_rs::TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        totp_secret.totp_secret,
        Some(state.app_name.clone()),
        state.app_name.clone(),
    )
    .context("error creating totp url")?
    .check(&secret.totp, Utc::now().timestamp() as u64)
    {
        true => {
            let refresh_jwt = ctx
                .with_valid_totp(true)
                .to_jwt(EncodingKey::from_secret(state.auth_secret.as_bytes()))?;
            refresh_cookie.set_value(refresh_jwt.clone());
            Ok((json!({"jwt":refresh_jwt,"ctx":ctx}).to_string()).into_response())
        }
        false => Ok(StatusCode::BAD_REQUEST.into_response()),
    }
}
