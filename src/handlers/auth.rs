use actix_web::{Responder, web, Result, error, http::{header::ContentType, StatusCode},
                App, HttpResponse,};
use serde::Serialize;
use serde::Deserialize;
use crate::{auth, db};
use derive_more::{Display, Error};


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



#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

pub async fn signup(data: web::Json<SignUpReqData>,  pool: web::Data<sqlx::PgPool>,) -> Result<impl Responder, MyError> {
    let p = pool.get_ref();
    println!("p = {:?}", p);

    if data.password != data.password_confirmation {
        return Err(MyError::BadClientData);
    }

    let email = data.email.clone();
    let mut user = db::users::fetch_optional(p, &email).await;
    if user.is_none() {
        user = Some(db::users::insert(p, &email).await);
    }

    let access_token = auth::jwt_encode(&user.unwrap());

    let resp_data = SignUpRespData { access_token };
    Ok(web::Json(resp_data))
}

