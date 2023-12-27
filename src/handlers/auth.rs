use actix_web::{HttpServer, Responder, web};
use serde::Serialize;
use crate::config::Config;
use crate::{auth, db};


#[derive(Serialize)]
struct SignInRespData {
    access_token: String,
}
pub async fn signin() -> impl Responder {
    let resp_data = SignInRespData {
        access_token: "qwe.asd.zxc".to_string(),
    };
    web::Json(resp_data)
}

pub async fn signup(pool: web::Data<sqlx::PgPool>,) -> impl Responder {
    let p = pool.get_ref();
    println!("p = {:?}", p);

    let users = db::users::fetch_all(p).await;

    let access_token = auth::jwt_encode(users.get(0).unwrap());

    let resp_data = SignInRespData { access_token };
    web::Json(resp_data)
}