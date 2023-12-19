mod router;

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
    let addr = get_address();
    let router = router::build_router();
    info!("router initialized, now listening on http://{}", addr);


    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router)
        .await
        .context("error while starting server")?;

    Ok(())
}

fn get_port() -> u16 {
    match dotenv::var("HTTPD_PORT") {
        Ok(port) => port.parse().unwrap_or(8080),
        Err(_) => {
            info!("PORT not set, defaulting to 8080");
            8080
        }
    }
}

fn get_address() -> std::net::SocketAddr {
    let port = get_port();
    match dotenv::var("HTTPD_ADDRESS") {
        Ok(address) => {
            info!("ADDRESS set to {}", address);
            address.parse().unwrap_or(([127, 0, 0, 1], port).into())
        }
        Err(_) => {
            let default_address = [127, 0, 0, 1];
            info!("ADDRESS not set, defaulting to {:?}", default_address);
            ([127, 0, 0, 1], port).into()
        }
    }
}