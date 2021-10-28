use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{MySqlPool, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

//pub async fn run() -> std::io::Result<()> {
//pub fn run() -> Result<Server, std::io::Error> {
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    my_pool: MySqlPool,
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    let my_pool = web::Data::new(my_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(my_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
