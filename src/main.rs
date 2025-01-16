use sqlx::sqlite::SqlitePoolOptions;
use wol_server::{configuration::load_settings, telemetry::*};

fn main() {
    println!("Hello, world!");
    let settings = load_settings().unwrap();
    let db_pool = SqlitePoolOptions::new().connect_lazy_with(settings.database.on_file());
    if settings.logging.enabled {
        let telemetry_subscriber = get_subscriber(
            "ticket_app".to_string(),
            settings.logging.level,
            std::io::Stdout,
        );
        init_subscriber(telemetry_subscriber);
    }
}
