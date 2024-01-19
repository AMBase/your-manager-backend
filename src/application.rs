use actix_web::{web, HttpServer};
use sqlx;

use crate::config::Config;
use crate::handlers;

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let pool = self.db_connect().await;

        let host = self.config.server.host.clone();
        let port = self.config.server.port.clone();

        println!("Server running on host {} port {}", &host, &port);
        HttpServer::new(move || {
            actix_web::App::new()
                .app_data(web::Data::new(pool.clone()))
                .route(
                    "/api/v1/auth/signin",
                    web::post().to(handlers::auth::signin),
                )
                .route(
                    "/api/v1/auth/signup",
                    web::post().to(handlers::auth::signup),
                )
                .route(
                    "/api/v1/userinfo",
                    web::get().to(handlers::users::userinfo),
                )
        })
            .bind((host, port))?
            .run()
            .await
    }

    async fn db_connect(&self) -> sqlx::PgPool {
        let conn_url = "postgres://postgres:postgres@localhost/postgres";
        let pool = match sqlx::PgPool::connect(&conn_url).await {
            Ok(pool) => pool,
            Err(sqlx::Error::PoolTimedOut) => panic!("Postgres timeout connection error"),
            Err(error) => panic!("Postgres connection error: {:?}", error),
        };
        pool
    }
}

#[derive(Default)]
pub struct AppBuilder {
    config: Config,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn build(self) -> App {
        App {
            config: self.config,
        }
    }
}
