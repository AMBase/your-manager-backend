use actix_web::{error, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::{auth, db};

#[derive(Serialize)]
struct SignInRespData {
    #[serde(rename(serialize = "accessToken", deserialize = "accessToken"))]
    access_token: String,
}

#[derive(Deserialize)]
pub struct SignInReqData {
    email: String,
    password: String,
}

pub async fn signin(
    pool: web::Data<sqlx::PgPool>,
    data: web::Json<SignInReqData>,
) -> Result<impl Responder> {
    let mut result = db::users::fetch_optional(&pool, &data.email).await;
    if result.is_none() {
        return Err(error::ErrorUnauthorized("Unauthorized"));
    }

    let user = result.unwrap();
    if data.password != user.password {
        return Err(error::ErrorUnauthorized("Unauthorized"));
    }

    let access_token = auth::jwt_encode(&user);
    let resp_data = SignInRespData { access_token };

    Ok(web::Json(resp_data))
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

pub async fn signup(
    data: web::Json<SignUpReqData>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder> {
    if data.password != data.password_confirmation {
        return Err(error::ErrorBadRequest("test"));
    }

    let email = data.email.clone();
    let mut user = db::users::fetch_optional(&pool, &email).await;
    if user.is_none() {
        user = Some(db::users::insert(&pool, &email, &data.password).await);
    }

    let access_token = auth::jwt_encode(&user.unwrap());

    let resp_data = SignUpRespData { access_token };
    Ok(web::Json(resp_data))
}
