use axum::response::IntoResponse;
use askama::Template;

pub async fn playground() -> impl IntoResponse {
    PlaygroundTemplate {
    }
}

#[derive(Template)]
#[template(path = "pages/playground.html")]
struct PlaygroundTemplate {
}
