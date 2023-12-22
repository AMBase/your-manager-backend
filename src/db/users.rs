use sqlx::PgPool;
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