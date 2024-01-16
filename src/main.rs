mod application;
mod auth;
mod config;
mod db;
mod handlers;

#[actix_web::main]
async fn main() {
    application::AppBuilder::new().build().run().await;
}
