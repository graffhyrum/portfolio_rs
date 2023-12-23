mod router;
mod address;

use dotenv::dotenv;
use anyhow::Context;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "portfolio_rs=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing Router");
    let addr = address::get_address();
    let router = router::build_router()?;
    info!("router initialized, now listening on http://{}", addr);


    let listener_result = tokio::net::TcpListener::bind(addr).await;
    let listener = listener_result.context("error while binding to address")?;
    axum::serve(listener, router)
        .await
        .context("error while starting server")?;

    Ok(())
}
