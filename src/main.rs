use askama::Template;
use anyhow::Context;
use axum::{http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};
use tower_http::services::ServeDir;
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

  let assets_path = std::env::current_dir().unwrap();
  let router = Router::new().route("/", get(hello)).nest_service(
    "/assets",
    ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
  );
  let port = 8000_u16;
  let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

  info!("router initialized, now listening on port {}", port);

  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, router).await.context("error while starting server")?;

  Ok(())
}

async fn hello() -> impl IntoResponse {
  let template = HelloTemplate {};
  HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
  where
    T: Template,
{
  fn into_response(self) -> Response {
    match self.0.render() {
      Ok(html) => Html(html).into_response(),
      Err(err) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to render template, Error: {}", err),
      )
        .into_response(),
    }
  }
}