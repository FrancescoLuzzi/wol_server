use crate::{
    app_state::SharedAppState, auth::password::hash_password, controller::error::GenericAuthError,
};
use anyhow::Context;
use axum::{extract::State, http::StatusCode, Form};
use rand::Rng as _;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct UserSignup {
    full_name: String,
    username: String,
    email: String,
    password: String,
    request_text: String,
}

pub async fn post(
    State(state): State<SharedAppState>,
    Form(signup): Form<UserSignup>,
) -> Result<StatusCode, GenericAuthError> {
    let mut transaction = state
        .db_pool
        .begin()
        .await
        .context("can't start transaction")?;
    let hashed_password = hash_password(&signup.password).await?;
    let totp_secret = rand::thread_rng().gen::<[u8; 21]>().to_vec();
    let user_id = Uuid::now_v7();
    //TODO: insert user infos
    sqlx::query_as!(
        NewUserUuid,
        r#"INSERT INTO users(id, roles, username, password, email, full_name, totp_secret)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        user_id,
        "user",
        signup.username,
        hashed_password,
        signup.email,
        signup.full_name,
        totp_secret
    )
    .execute(&mut *transaction)
    .await
    .context("Failed new user subscription")?;

    //TODO: insert user request for admin
    sqlx::query!(
        "INSERT INTO users_signup_requests (user_id, request_text) VALUES ($1, $2)",
        user_id,
        signup.request_text,
    )
    .execute(&mut *transaction)
    .await
    .context("can't add user request")?;

    transaction
        .commit()
        .await
        .context("failed committing transaction")?;
    Ok(StatusCode::OK)
}
