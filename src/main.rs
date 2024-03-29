//! src/main.rs
//df use sqlx::postgres::PgPool;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    //let connection_pool = PgPool::connect(&configuration.database.connection_string())
    //let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let connection_pool_mysql = MySqlPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.mysql.connection_string_mysql())
        .await
        .expect("Failed to connect to Mysql.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address).expect("Failed to bind 8080 port");
    run(listener, connection_pool, connection_pool_mysql)?.await
}
