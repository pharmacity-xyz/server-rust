use crate::{
    configuration::{DatabaseSettings, Settings},
    routes::{
        auth::{change_password::change_password, login::login},
        carts::{get_all_carts, post_cart, update_cart},
        categories::{get_categories, post_category, update_category},
        health_check, post_order,
        products::{
            get_all_products, get_featured_products, get_product_by_categoryid,
            get_product_by_productid, post_product, search_product, update_product,
        },
        users::{get_all_users, post_user, update_user},
    },
};
use actix_cors::Cors;
use actix_web::{cookie::Key, dev::Server, http::header, web, web::Data, App, HttpServer};
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
        .acquire_timeout(std::time::Duration::from_secs(10))
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
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/health_check", web::get().to(health_check))
            .route("/auth/register", web::post().to(post_user))
            .route("/auth/login", web::post().to(login))
            .route("/auth/change_password", web::post().to(change_password))
            .route("/user", web::get().to(get_all_users))
            .route("/user", web::put().to(update_user))
            .route("/category", web::post().to(post_category))
            .route("/category", web::get().to(get_categories))
            .route("/category", web::put().to(update_category))
            .route("/product", web::post().to(post_product))
            .route("/product", web::get().to(get_all_products))
            .route("/product/product", web::get().to(get_product_by_productid))
            .route(
                "/product/category",
                web::get().to(get_product_by_categoryid),
            )
            .route("/product/featured", web::get().to(get_featured_products))
            .route("/product/search", web::get().to(search_product))
            .route("/product", web::put().to(update_product))
            .route("/cart/add", web::post().to(post_cart))
            .route("/cart", web::get().to(get_all_carts))
            .route("/cart/update_quantity", web::put().to(update_cart))
            .route("/orders", web::post().to(post_order))
            .app_data(db_pool.clone())
            .app_data(Data::new(hmac_secret.clone()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
