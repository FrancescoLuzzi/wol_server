mod auth;
use crate::app_state::SharedAppState;

pub fn route() -> axum::Router<SharedAppState> {
    axum::Router::new().nest("/auth", auth::routes())
}
