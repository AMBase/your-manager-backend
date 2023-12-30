use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
}
pub async fn fetch_all(pool: &PgPool) -> Vec<User> {
    let mut users = vec![];

    let rows = sqlx::query("SELECT * FROM users").fetch_all(pool).await.unwrap();
    for row in rows {
        users.push(User {
            id: row.get("id"),
            email: row.get("email"),
        });
    }

    return users;
}

pub async fn fetch_optional(pool: &PgPool, email: String) -> Option<User> {
    let mut result = None;

    let row = sqlx::query("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool).await.unwrap();

    return result;
}
