use super::ctx::Ctx;
use crate::model::role::Role;
use crate::{auth::error::AuthError, telemetry::spawn_blocking_with_tracing};
use anyhow::Context;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use argon2::{PasswordHash, PasswordVerifier};
use rand;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn is_password_strong(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }
    let mut score = 1;

    for c in password.chars() {
        if c.is_lowercase() {
            score |= 0b0010;
            continue;
        }
        if c.is_uppercase() {
            score |= 0b0100;
            continue;
        }
        if c.is_ascii_digit() {
            score |= 0b1000;
            continue;
        }
    }
    score == 15
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_user_credentials(
    username: &str,
    pool: &SqlitePool,
) -> Result<(Ctx, String), anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id as "id: uuid::Uuid", roles , password
        FROM users
        WHERE email = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to performed a query to retrieve stored credentials.")?
    .ok_or(AuthError::InvalidCredentials(anyhow::anyhow!(
        "Unknown username."
    )))?;
    Ok((
        Ctx::new(row.id, Role::parse_roles(&row.roles).expect("we bad roles")),
        row.password,
    ))
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &SqlitePool,
) -> Result<Ctx, AuthError> {
    // TODO: return User instead of user_id
    let mut user = None;
    let mut expected_password_hash = "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
        .to_string();

    if let Ok((stored_user, stored_password_hash)) =
        get_user_credentials(&credentials.email, pool).await
    {
        user = Some(stored_user);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user.ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_ref())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "hash_password_sync", skip(password))]
pub fn hash_password_sync(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();
    Ok(password_hash)
}

pub async fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let password = password.to_string();
    spawn_blocking_with_tracing(move || hash_password_sync(&password))
        .await?
        .context("Failed to hash password")
}
