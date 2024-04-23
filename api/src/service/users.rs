use std::sync::Arc;

use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct Users {
    pool: Arc<Pool<Postgres>>,
}

#[derive(Debug, FromRow, Clone, Deserialize, Serialize, Type)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

impl Users {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let query = "SELECT id, email, password FROM users WHERE email = $1";
        let user = sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let query = "SELECT id, email, password FROM users WHERE id = $1";
        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(&self, email: &str, password: &str) -> Result<User, sqlx::Error> {
        let query =
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email, password";
        let user = sqlx::query_as::<_, User>(query)
            .bind(email)
            .bind(password)
            .fetch_one(&*self.pool)
            .await?;

        Ok(user)
    }
}
