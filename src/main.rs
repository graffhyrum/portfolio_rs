mod router;

use anyhow::Context;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "portfolio_rs=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing Router");
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let router = router::build_router();
    info!("router initialized, now listening on addr http://{}", addr);


    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router)
        .await
        .context("error while starting server")?;

    Ok(())
}
