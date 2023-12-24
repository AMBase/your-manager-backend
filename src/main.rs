mod config;
mod db;
mod auth;
mod app;






#[actix_web::main]
async fn main() {
    app::App::new().run().await;
}
