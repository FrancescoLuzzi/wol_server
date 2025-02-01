use axum::{
    self,
    http::{self, header, HeaderValue},
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use tower_http::{cors, services::ServeDir};
use wol_server::{
    app_state::{AppState, SharedAppState},
    configuration::load_settings,
    controller::app,
    controller::health_check,
    migration::db_migration,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let settings = load_settings(&base_path.join("configuration")).unwrap();
    if settings.logging.enabled {
        let telemetry_subscriber = get_subscriber(
            "wol_server".to_string(),
            settings.logging.level,
            std::io::stdout,
        );
        init_subscriber(telemetry_subscriber);
    }
    let db_pool = SqlitePoolOptions::new().connect_lazy_with(settings.database.on_file());
    db_migration(&db_pool).await.expect("can't run migrations");
    let app_state = SharedAppState::new(AppState {
        base_url: settings.application.base_url,
        db_pool: db_pool,
        hmac_secret: settings.application.hmac_secret,
    });

    let serve_dir = ServeDir::new("dist");

    let app = Router::new()
        // .route("/admin/user_requests", get(admin::get_user_requests)) // TODO: add pagination
        // .route("/admin/user_requests/{id}", get(admin::get_user_request_by_id))
        // .route("/admin/user_requests/{id}/reject", post(admin::post_accept_user_requests))
        // .route("/admin/user_requests/{id}/accept", post(admin::post_reject_user_requests))
        // .route_layer(middleware::from_fn(mw_auth::mw_ctx_require_admin))
        // .route("/devices", get(device::get)) // TODO: add pagination
        // .route("/devices", post(device::post))
        // .route("/devices/{id}", post(device::get_by_id))
        // .route("/devices/{id}", delete(device::delete_by_id))
        // .route("/devices/{id}/refresh", get(device::get_refresh_by_id)) // TODO: add rate limiting
        // .route("/devices/{id}/power_on", post(device::post_power_on_by_id))
        // .route_layer(middleware::from_fn(mw_auth::mw_ctx_require))
        .route("/auth/logout", post(app::auth::logout::post))
        .route("/auth/login", post(app::auth::login::post))
        .route("/auth/signup", post(app::auth::signup::post))
        .route("/auth/refresh", get(app::auth::refresh::get))
        .route("/auth/totp", get(app::auth::totp::get_regenerate))
        .route("/auth/totp", post(app::auth::totp::post))
        .route("/health_check", get(health_check::get))
        .layer(CookieManagerLayer::new())
        .layer(
            cors::CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
                .allow_credentials(true)
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]),
        )
        .nest_service("/dist", serve_dir)
        .with_state(app_state);

    let addr = SocketAddr::new(settings.application.host, settings.application.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
