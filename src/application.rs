use actix_web::{HttpServer, web};
use actix_web::dev::Response;

use crate::config::Config;

use crate::handlers;
use sqlx;
use sqlx::{Database, Error};


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
                .route("/api/v1/auth/signin", web::post().to(handlers::auth::signin))
                .route("/api/v1/auth/signup", web::post().to(handlers::auth::signup))
        })
            .bind((host, port))?
            .run()
            .await
    }


    async fn db_connect(&self) -> sqlx::PgPool {
        let conn_url = "postgres://postgres:postgres@localhost/postgres";
        let pool = sqlx::PgPool::connect(&conn_url).await.unwrap();
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
        App { config: self.config }
    }
}