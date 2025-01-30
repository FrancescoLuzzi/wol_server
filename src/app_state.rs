use sqlx::SqlitePool;
use std::sync::Arc;

pub type SharedAppState = Arc<AppState>;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub hmac_secret: String,
    pub base_url: String,
}
