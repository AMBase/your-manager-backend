use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgRow;
use sqlx::Row;
use crate::domain::aggregates::User;

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
        })
    }
}


pub async fn fetch_all(pool: &PgPool) -> Vec<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn fetch_optional(pool: &PgPool, email: &String) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .unwrap()
}

pub async fn insert(pool: &PgPool, email: &String, password: &String) -> User {
    sqlx::query_as::<_, User>("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *")
        .bind(email.clone())
        .bind(password.clone())
        .fetch_one(pool)
        .await
        .unwrap()
}

pub async fn get_by_id(pool: &PgPool, id: &i32) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap()
}
