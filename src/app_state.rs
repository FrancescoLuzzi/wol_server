use sqlx::SqlitePool;
use std::sync::Arc;

pub type SharedAppState = Arc<AppState>;
pub type SharedAuthState = Arc<AuthState>;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub hmac_secret: String,
    pub base_url: String,
}

pub struct AuthState {
    pub hmac_secret: String,
}
