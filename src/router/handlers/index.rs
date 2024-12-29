use askama::Template;
use axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {}
