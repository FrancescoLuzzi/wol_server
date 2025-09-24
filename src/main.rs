use axum::{
    http::{self, header, HeaderValue, StatusCode, Uri},
    middleware,
    response::{Html, IntoResponse as _, Response},
    routing::{get, post},
    serve, Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors;
use wol_server::{
    app_state::{AppState, SharedAppState},
    configuration::load_settings,
    controller::{app, health_check},
    middleware::mw_auth,
    migration::db_migration,
    telemetry::{get_subscriber, init_subscriber},
};

const INDEX_HTML: &str = "index.html";

#[derive(rust_embed::Embed)]
#[folder = "frontend/dist"]
struct Assets;

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => (
            [(header::CONTENT_TYPE, content.metadata.mimetype())],
            content.data,
        )
            .into_response(),
        None => {
            if path.contains('.') || path.starts_with("api") {
                return not_found().await;
            }
            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

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
        db_pool,
        auth_secret: settings.application.auth_secret,
        app_name: settings.application.app_name,
    });

    // let serve_dir = ServeDir::new("frontend/dist");
    let serve_dir = get(static_handler);

    let app = Router::new()
        // .route("/api/admin/user_requests", get(admin::get_user_requests)) // TODO: add pagination
        // .route("/api/admin/user_requests/{id}", get(admin::get_user_request_by_id))
        // .route("/api/admin/user_requests/{id}/reject", post(admin::post_accept_user_requests))
        // .route("/api/admin/user_requests/{id}/accept", post(admin::post_reject_user_requests))
        // .route("/api/devices", get(device::get)) // TODO: add pagination
        // .route("/api/devices", post(device::post))
        // .route("/api/devices/{id}", post(device::get_by_id))
        // .route("/api/devices/{id}", delete(device::delete_by_id))
        // .route("/api/devices/{id}/refresh", get(device::get_refresh_by_id)) // TODO: add rate limiting
        // .route("/api/devices/{id}/power_on", post(device::post_power_on_by_id))
        .route(
            "/api/auth/totp/regenerate",
            get(app::auth::totp::get_regenerate),
        )
        .route(
            "/api/auth/totp/validate",
            post(app::auth::totp::post_validate),
        )
        .route("/api/auth/totp", post(app::auth::totp::post))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            mw_auth::user_must_be_active,
        ))
        .route("/api/auth/signup", post(app::auth::signup::post))
        .route("/api/auth/refresh", get(app::auth::refresh::get))
        .route("/api/auth/logout", post(app::auth::logout::post))
        .route("/api/auth/login", post(app::auth::login::post))
        .layer(CookieManagerLayer::new())
        .route("/api/health_check", get(health_check::get))
        .layer(
            cors::CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
                .allow_credentials(true)
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]),
        )
        .fallback_service(serve_dir)
        .with_state(app_state);

    let addr = SocketAddr::new(settings.application.host, settings.application.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", addr);
    serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
