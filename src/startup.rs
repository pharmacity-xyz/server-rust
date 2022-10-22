use crate::{
    configuration::{DatabaseSettings, Settings},
    routes::{
        health_check,
        users::{change_password, login::login, post::post_user},
    },
};
use actix_web::{cookie::Key, dev::Server, web, web::Data, App, HttpServer};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            configuration.application.hmac_secret,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    hmac_secret: Secret<String>,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/users", web::post().to(post_user))
            .route("/users/login", web::post().to(login))
            .route("/users/change_password", web::post().to(change_password))
            .app_data(db_pool.clone())
            .app_data(Data::new(hmac_secret.clone()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
