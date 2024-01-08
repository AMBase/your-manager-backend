use actix_web::{Responder, web, HttpResponse};
use serde::Serialize;
use serde::Deserialize;
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

#[derive(Serialize)]
struct ErrorRespData {
    code: i32,
    message: String,
}

#[derive(Serialize)]
pub struct SignUpRespData {
    access_token: String,
}
#[derive(Deserialize)]
pub struct SignUpReqData {
    email: String,
    password: String,
    password_confirmation: String,
}
pub async fn signup(data: web::Json<SignUpReqData>,  pool: web::Data<sqlx::PgPool>,) -> web::Json<impl Responder> {
    let p = pool.get_ref();
    println!("p = {:?}", p);

    if data.password != data.password_confirmation {
        return web::Json(ErrorRespData {
            code: 0,
            message: "Incorrect password".to_string(),
        });
    }

    let email = data.email.clone();
    let mut user = db::users::fetch_optional(p, &email).await;
    if user.is_none() {
        user = Some(db::users::insert(p, &email).await);
    }

    let access_token = auth::jwt_encode(&user.unwrap());

    let resp_data = SignUpRespData { access_token };
    web::Json(resp_data)
}

