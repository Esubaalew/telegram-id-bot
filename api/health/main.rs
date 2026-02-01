use vercel_runtime::{run, Body, Error, Request, Response};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let health_info = json!({
        "status": "healthy",
        "service": "Telegram ID Analyzer Bot",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0",
        "runtime": "Rust on Vercel"
    });

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(health_info.to_string().into())?)
}