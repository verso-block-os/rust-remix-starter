use std::{error::Error, sync::Arc};

use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct Todos {
    pool: Arc<Pool<Postgres>>,
}

#[derive(Debug, FromRow, Clone, Deserialize, Serialize, Type)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todos {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, title: &str) -> Result<Todo, Box<dyn Error>> {
        let query = "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed";
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(title)
            .fetch_one(&*self.pool)
            .await?;

        Ok(todo)
    }

    pub async fn toggle(&self, id: i32) -> Result<Todo, Box<dyn Error>> {
        let query = "UPDATE todos SET completed = NOT completed WHERE id = $1 RETURNING id, title, completed";
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(todo)
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let query = "DELETE FROM todos WHERE id = $1";
        sqlx::query(query).bind(id).execute(&*self.pool).await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let query = "SELECT id, title, completed FROM todos ORDER BY id";
        let todos = sqlx::query_as::<_, Todo>(query)
            .fetch_all(&*self.pool)
            .await?;

        Ok(todos)
    }
}
