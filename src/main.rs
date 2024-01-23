mod application;
mod auth;
mod config;
mod db;
mod handlers;
mod domain;

#[actix_web::main]
async fn main() {
    application::AppBuilder::new().build().run().await;
}
