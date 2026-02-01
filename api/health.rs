use serde_json::{json, Value};
use vercel_runtime::{Error, Request, service_fn, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(handler);
    run(service).await
}

async fn handler(_req: Request) -> Result<Value, Error> {
    Ok(json!({
        "status": "healthy",
        "service": "Telegram ID Analyzer Bot",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0",
        "runtime": "Rust on Vercel"
    }))
}