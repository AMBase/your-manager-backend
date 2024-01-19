use actix_web::{error, web, HttpRequest, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::{auth, db};

#[derive(Serialize)]
struct UserinfoRespData {
    id: i32,
    email: String,
}


pub async fn userinfo(
    request: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder> {
    let p = pool.get_ref();
    println!("p = {:?}", p);

    let authorization = request.headers().get("Authorization");
    if authorization.is_none() {
        return Err(error::ErrorUnauthorized("Unauthorized"));
    }

    // let mut user = db::users::fetch_optional(p, &email).await;
    // if user.is_none() {
    //     user = Some(db::users::insert(p, &email).await);
    // }
    //
    // let access_token = auth::jwt_encode(&user.unwrap());

    let resp_data = UserinfoRespData { id: 1, email: "test".to_string() };
    Ok(web::Json(resp_data))
}
