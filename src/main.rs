mod config;
use config::Config;

use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
#[derive(Serialize)]
struct SignInRespData {
    access_token: String,
}



async fn signin() -> impl Responder {
    let resp_data = SignInRespData {
        access_token: "qwe.asd.zxc".to_string(),
    };
    web::Json(resp_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    println!("Server running on host {} port {}", config.server.host, config.server.port);
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/auth/signin", web::post().to(signin))
    })
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}