use sqlx::{Pool, Postgres};
use std::{env, error::Error, sync::Arc};

pub mod todos;

pub async fn connect() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[derive(Clone, Debug)]
pub struct Database {
    pub todos: todos::Todos,
}

impl Database {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        let todos = todos::Todos::new(pool.clone());

        Self { todos }
    }
}
