use axum::response::IntoResponse;
use askama::Template;

pub async fn index() -> impl IntoResponse {
    IndexTemplate {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    version: String,
}
