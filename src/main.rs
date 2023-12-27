mod config;
mod db;
mod auth;
mod application;
mod handlers;


#[actix_web::main]
async fn main() {
    let mut app = application::AppBuilder::new().build();
    app.run().await;
}
