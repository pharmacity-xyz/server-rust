use crate::routes::{
    health_check,
    users::{login::login, post::post_user},
};
use actix_web::{cookie::Cookie, dev::Server};
use actix_web::{web, web::Data, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let message_store = CookieMessageStore::builder(todo!()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(message_framework.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/users", web::post().to(post_user))
            .route("/users/login", web::post().to(login))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
