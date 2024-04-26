use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};

use core::context::{self, Context};
use rspc::integrations::httpz::Request;
use std::{
    env,
    error::Error,
    net::{Ipv6Addr, SocketAddr},
    sync::{Arc, Mutex},
};
use tower_http::cors::CorsLayer;

mod core;
mod middleware;
mod router;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Failed to read .env file");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let public_domain = env::var("PUBLIC_DOMAIN").expect("PUBLIC_DOMAIN must be set");

    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 4000));

    let pool = Arc::new(pool);

    let auth = service::auth::Auth::new(pool.clone());
    let users = service::users::Users::new(pool.clone());
    let todos = service::todos::Todos::new(pool.clone());

    let router = router::get();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(public_domain.parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .nest(
            "/",
            router
                .endpoint(|req: Request| {
                    let mut ctx = Context::new();
                    context::add!(ctx, Mutex::new(req));
                    context::add!(ctx, auth);
                    context::add!(ctx, users);
                    context::add!(ctx, todos);
                    ctx
                })
                .axum(),
        )
        .layer(cors);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
