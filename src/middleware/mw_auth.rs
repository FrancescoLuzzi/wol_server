use anyhow::Context;
use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    app_state::SharedAppState,
    auth::{ctx::Ctx, error::AuthError},
};

pub async fn user_must_be_active(
    State(state): State<SharedAppState>,
    ctx: Ctx,
    req: Request<Body>,
    next: Next,
) -> Result<Response, AuthError> {
    let is_active = sqlx::query!("SELECT active FROM users WHERE id = $1", ctx.user_id)
        .fetch_optional(&state.db_pool)
        .await
        .context("Can't fetch user active status.")?
        .ok_or(AuthError::InactiveUser)?
        .active;
    match is_active {
        true => Ok(next.run(req).await),
        false => Err(AuthError::InactiveUser),
    }
}
