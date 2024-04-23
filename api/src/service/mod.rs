use sqlx::{Pool, Postgres};
use std::{env, error::Error, sync::Arc};

mod auth;
mod todos;
mod users;

pub async fn connect() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[derive(Clone, Debug)]
pub struct Service {
    pub todos: todos::Todos,
    pub auth: auth::Auth,
    pub users: users::Users,
}

impl Service {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        let todos = todos::Todos::new(pool.clone());
        let auth = auth::Auth::new(pool.clone());
        let users = users::Users::new(pool.clone());

        Self { todos, auth, users }
    }
}
