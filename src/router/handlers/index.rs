use axum::response::IntoResponse;
use askama::Template;

pub async fn index() -> impl IntoResponse {
    IndexTemplate {
    }
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
}
