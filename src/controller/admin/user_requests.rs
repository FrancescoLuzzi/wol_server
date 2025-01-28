// .route("/admin/user_requests", get(admin::get_user_requests)) // TODO: add pagination
// .route("/admin/user_requests/{id}", get(admin::get_user_request_by_id))
// .route("/admin/user_requests/{id}/reject", post(admin::post_accept_user_requests))
// .route("/admin/user_requests/{id}/accept", post(admin::post_reject_user_requests))

use crate::{
    app_state::SharedAppState, auth::ctx::Ctx, controller::error::UnknownError,
    model::user_request::UserSignupRequest,
};
use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use chrono::NaiveDateTime;
use uuid::Uuid;

pub async fn get(
    State(state): State<SharedAppState>,
    Extension(ctx): Extension<Ctx>,
) -> Result<Vec<UserSignupRequest>, UnknownError> {
    dbg!("querying user signup requests as {}", ctx.user_id);
    let signups = sqlx::query_as!(
        UserSignupRequest,
        r#"SELECT user_id as "user_id: Uuid",request_text from users_signup_requests"#
    )
    .fetch_all(&state.db_pool)
    .await
    .context("can't query user signup requests")?;
    Ok(signups)
}

pub async fn get_accept(
    State(state): State<SharedAppState>,
    Extension(ctx): Extension<Ctx>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, UnknownError> {
    dbg!("accepting user signup requests as {}", ctx.user_id);
    let mut transaction = state
        .db_pool
        .begin()
        .await
        .context("can't start transaction")?;

    sqlx::query!(
        r#"DELETE FROM users_signup_requests WHERE user_id=$1"#,
        user_id
    )
    .execute(&mut *transaction)
    .await
    .context("can't accept uuid request")?;

    sqlx::query!(
        r#"UPDATE users SET active=1, join_date=datetime('now','localtime') WHERE id=$1"#,
        user_id
    )
    .execute(&mut *transaction)
    .await
    .context("can't accept user")?;

    transaction
        .commit()
        .await
        .context("can't commit transaction")?;
    Ok(StatusCode::OK)
}

#[derive(serde::Deserialize)]
struct UserInfos {
    username: String,
    email: String,
    request_date: NaiveDateTime,
}

pub async fn get_reject(
    State(state): State<SharedAppState>,
    Extension(ctx): Extension<Ctx>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, UnknownError> {
    dbg!("rejecting user signup requests as {}", ctx.user_id);
    let mut transaction = state
        .db_pool
        .begin()
        .await
        .context("can't start transaction")?;

    sqlx::query!(
        r#"DELETE FROM users_signup_requests WHERE user_id=$1"#,
        user_id
    )
    .execute(&mut *transaction)
    .await
    .context("can't accept uuid request")?;

    let user_infos = sqlx::query_as!(
        UserInfos,
        r#"DELETE FROM users WHERE id=$1 RETURNING username, email, request_date"#,
        user_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("can't accept user")?;

    sqlx::query!(
        r#"INSERT INTO user_rejections(username,email,request_date,rejection_date) VALUES ($1,$2,$3,datetime('now','localtime'))"#,
        user_infos.username,
        user_infos.email,
        user_infos.request_date
    ).execute(&mut *transaction)
    .await
    .context("can' reject user")?;

    transaction
        .commit()
        .await
        .context("can't commit transaction")?;

    Ok(StatusCode::OK)
}
