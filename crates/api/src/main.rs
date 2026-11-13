use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Health { status: &'static str }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let app = Router::new().route("/health", get(|| async { Json(Health { status: "ok" }) }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
