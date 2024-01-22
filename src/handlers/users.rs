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

    let header_value = authorization.unwrap();
    let token = header_value.to_str().unwrap_or("").replacen("Bearer ", "", 1);
    println!("token = {:?}", token);

    let token_data = auth::jwt_decode(&token);
    let user_id = token_data.get("sub");
    if user_id.is_none() {
        return Err(error::ErrorUnauthorized("Unauthorized"));
    }

    let result = db::users::get_by_id(p, &user_id.unwrap()).await;
    if result.is_none() {
        return Err(error::ErrorUnauthorized("Unauthorized"));
    }
    let user = result.unwrap();

    let resp_data = UserinfoRespData { id: user.id, email: user.email };

    Ok(web::Json(resp_data))
}
