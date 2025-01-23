use axum::{
    middleware,
    routing::{get, post},
};

use crate::{app_state::SharedAppState, auth::mw_auth};

mod login;
mod logout;
mod refresh;
mod totp;

pub fn routes() -> axum::Router<SharedAppState> {
    axum::Router::new()
        .route("/refresh", post(refresh::post))
        .route("/totp", get(totp::get))
        .route("/totp", post(totp::post))
        .route_layer(middleware::from_fn(mw_auth::mw_ctx_require))
        .route("/login", post(login::post))
        .route("/logout", post(logout::post))
}
