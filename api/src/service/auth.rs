use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, Pool, Postgres};
use std::{error::Error, sync::Arc};

#[derive(Clone, Debug)]
pub struct Auth {
    pool: Arc<Pool<Postgres>>,
}

#[derive(Debug, FromRow, Clone)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
    pub token: String,
}

#[derive(Debug, FromRow, Clone)]
pub struct SessionOnlyToken {
    pub token: String,
}

impl Auth {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn create_session(&self, user_id: i32) -> Result<String, Box<dyn Error>> {
        self.delete_expired_sessions(user_id).await?;

        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::days(7);
        let query =
            "INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING token";
        let session = sqlx::query_as::<_, SessionOnlyToken>(query)
            .bind(user_id)
            .bind(token)
            .bind(expires_at)
            .fetch_one(&*self.pool)
            .await?;

        Ok(session.token)
    }

    pub async fn delete_expired_sessions(&self, user_id: i32) -> Result<(), Box<dyn Error>> {
        let query = "DELETE FROM sessions WHERE user_id = $1 AND expires_at < NOW()";

        sqlx::query(query)
            .bind(user_id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn invalidate_session(&self, token: &str) -> Result<(), Box<dyn Error>> {
        let query = "DELETE FROM sessions WHERE token = $1";

        sqlx::query(query).bind(token).execute(&*self.pool).await?;

        Ok(())
    }

    pub async fn get_session(&self, token: &str) -> Result<Option<Session>, Box<dyn Error>> {
        let query = "SELECT id, user_id, expires_at, token FROM sessions WHERE token = $1";
        let session = sqlx::query_as::<_, Session>(query)
            .bind(token)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(session)
    }

    pub async fn validate_session(&self, token: &str) -> Result<Option<Session>, Box<dyn Error>> {
        let session = self.get_session(token).await?;

        if let Some(session) = session {
            if session.expires_at < Utc::now() {
                self.invalidate_session(token).await?;
                return Ok(None);
            }

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }
}
