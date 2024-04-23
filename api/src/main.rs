use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};
use database::todos::Todos;
use std::{env, error::Error, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod database;
mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Failed to read .env file");

    let public_domain = env::var("PUBLIC_DOMAIN").unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(public_domain.parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    let pool = Arc::new(database::connect().await?);
    let todos = Arc::new(Todos::new(pool));

    let router = router::get_router();
    let rpc = rspc_axum::endpoint(router, move || router::Context { todos });

    let app = Router::new()
        .nest("/rpc", rpc)
        .layer(ServiceBuilder::new().layer(cors))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
