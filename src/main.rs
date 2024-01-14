mod config;
mod db;
mod auth;
mod application;
mod handlers;


#[actix_web::main]
async fn main() {
    application::AppBuilder::new().build().run().await;
}
