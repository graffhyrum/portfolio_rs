use axum::response::IntoResponse;
use askama::Template;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    version: String,
}
