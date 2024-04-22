use std::{env, error::Error, sync::Arc};

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{prelude::FromRow, Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct Todos {
    pool: Pool<Postgres>,
}
#[derive(Debug, FromRow, Clone, Deserialize, Serialize, Type)]

struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todos {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, title: &str) -> Result<Todo, Box<dyn Error>> {
        let query = "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed";
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(title)
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
    }

    pub async fn toggle(&self, id: i32) -> Result<Todo, Box<dyn Error>> {
        let query = "UPDATE todos SET completed = NOT completed WHERE id = $1 RETURNING id, title, completed";
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let query = "DELETE FROM todos WHERE id = $1";
        sqlx::query(query).bind(id).execute(&self.pool).await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let query = "SELECT id, title, completed FROM todos ORDER BY id";
        let todos = sqlx::query_as::<_, Todo>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(todos)
    }
}

struct Context {
    todos: Arc<Todos>,
}

async fn index() -> &'static str {
    "App is running!"
}

fn get_router() -> Arc<rspc::Router<Context>> {
    rspc::Router::<Context>::new()
        .query("version", |t| t(|_ctx: Context, _: ()| "1.0.0"))
        .query("getTodos", |t| {
            t(|ctx: Context, _: ()| async move {
                let todos = ctx.todos.get_all().await.unwrap();

                Ok(todos)
            })
        })
        .mutation("createTodo", |t| {
            t(|ctx: Context, title: String| async move {
                let todo = ctx.todos.create(&title).await.unwrap();

                Ok(todo)
            })
        })
        .mutation("toggleTodo", |t| {
            t(|ctx: Context, id: i32| async move {
                let todo = ctx.todos.toggle(id).await.unwrap();

                Ok(todo)
            })
        })
        .mutation("deleteTodo", |t| {
            t(|ctx: Context, id: i32| async move {
                ctx.todos.delete(id).await.unwrap();

                Ok(())
            })
        })
        .config(rspc::Config::new().export_ts_bindings("../web/app/generated/bindings.ts"))
        .build()
        .arced()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Failed to read .env file");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    let url = env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let todos = Arc::new(Todos::new(pool));

    let router = get_router();
    let rpc = rspc_axum::endpoint(router, move || Context { todos });

    let app = Router::new()
        .nest("/rpc", rpc)
        .route("/", get(index))
        .layer(ServiceBuilder::new().layer(cors));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
