use crate::{auth::error::AuthError, model::user::User, telemetry::spawn_blocking_with_tracing};
use anyhow::Context;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use argon2::{PasswordHash, PasswordVerifier};
use rand;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: SecretString,
}

pub fn is_password_strong(password: &SecretString) -> bool {
    let password = password.expose_secret();
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
async fn get_user_by_email(
    username: &str,
    pool: &SqlitePool,
) -> Result<Option<(User, SecretString)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id as "id: uuid::Uuid", password
        FROM users
        WHERE email = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to performed a query to retrieve stored credentials.")?
    .map(|row| (row.id, SecretString::from(row.password)));
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &SqlitePool,
) -> Result<uuid::Uuid, AuthError> {
    // TODO: return User instead of user_id
    let mut user = None;
    let mut expected_password_hash = SecretString::from(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_user_by_email(&credentials.email, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "hash_password_sync", skip(password))]
pub fn hash_password_sync(password: SecretString) -> Result<SecretString, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();
    Ok(SecretString::from(password_hash))
}

pub async fn hash_password(password: SecretString) -> Result<SecretString, anyhow::Error> {
    spawn_blocking_with_tracing(move || hash_password_sync(password))
        .await?
        .context("Failed to hash password")
}
