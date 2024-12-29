use askama::Template;
use axum::response::IntoResponse;

pub async fn playground() -> impl IntoResponse {
    PlaygroundTemplate {}
}

#[derive(Template)]
#[template(path = "pages/playground.html")]
struct PlaygroundTemplate {}
