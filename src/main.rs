mod config;
use config::Config;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
#[derive(Serialize)]
struct SignInRespData {
    access_token: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/api/v1/auth/signin", web::post().to(signin))
    })
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}