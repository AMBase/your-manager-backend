use sqlx::PgPool;
use sqlx::Row;

#[derive(Debug, sqlx::FromRow)]
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

pub async fn fetch_optional(pool: &PgPool, email: &String) -> Option<User> {
    sqlx::query_as::<_,User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool).await.unwrap()
}

pub async fn insert(pool: &PgPool, email: &String) -> User {
    let result = sqlx::query("INSERT INTO users (email) VALUES ($1) RETURNING *")
        .bind(email.clone())
        .fetch_one(pool).await.unwrap();

    User {
        id: result.get("id"),
        email: result.get("email"),
    }
}
