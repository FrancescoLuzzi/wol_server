use sqlx::SqlitePool;
use std::sync::Arc;

pub type SharedAppState = Arc<AppState>;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub auth_secret: String,
    pub base_url: String,
    pub app_name: String,
}
