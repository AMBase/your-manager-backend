mod config;
mod db;
mod auth;
mod application;






#[actix_web::main]
async fn main() {
    let mut app = application::AppBuilder::new().build();
    app.run().await;
}
