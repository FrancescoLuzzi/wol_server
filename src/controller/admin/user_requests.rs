// .route("/admin/user_requests", get(admin::get_user_requests)) // TODO: add pagination
// .route("/admin/user_requests/{id}", get(admin::get_user_request_by_id))
// .route("/admin/user_requests/{id}/reject", post(admin::post_accept_user_requests))
// .route("/admin/user_requests/{id}/accept", post(admin::post_reject_user_requests))

use crate::{
    app_state::SharedAppState,
    model::{ctx::Ctx, user_request::UserSignupRequest},
};
use axum::{extract::State, http::StatusCode, Extension};
use uuid::Uuid;

pub async fn get_user_requests(
    State(state): State<SharedAppState>,
    Extension(ctx): Extension<Ctx>,
) -> Result<Vec<UserSignupRequest>, StatusCode> {
    dbg!("querying user signup requests as {}", ctx.user_id);
    sqlx::query_as!(
        UserSignupRequest,
        r#"SELECT user_id as "user_id: Uuid",request_text from users_signup_requests"#
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
