use actix_web::{HttpServer, Responder, web};
use serde::Serialize;
use crate::config::Config;
use crate::{auth, db};

use sqlx;




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

async fn signup(pool: web::Data<sqlx::PgPool>,) -> impl Responder {
    let p = pool.get_ref();
    println!("p = {:?}", p);

    let users = db::users::fetch_all(p).await;

    let access_token = auth::jwt_encode(users.get(0).unwrap());

    let resp_data = SignInRespData { access_token };
    web::Json(resp_data)
}

pub struct App {
    config: Option<Config>,

}

impl App {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        self.read_config();
        let config = self.config.clone().unwrap();

        let conn_url = "postgres://postgres:postgres@localhost/postgres";
        let pool = sqlx::PgPool::connect(&conn_url).await.unwrap();

        println!("Server running on host {} port {}", config.server.host, config.server.port);
        HttpServer::new(move || {
            actix_web::App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/v1/auth/signin", web::post().to(signin))
                .route("/api/v1/auth/signup", web::post().to(signup))
        })
            .bind((config.server.host, config.server.port))?
            .run()
            .await
    }

    fn read_config(&mut self) {
        let config = Config::new();
        self.config = Some(config);
    }
}