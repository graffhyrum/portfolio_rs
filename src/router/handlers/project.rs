use askama::Template;
use axum::response::IntoResponse;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn projects() -> impl IntoResponse {
    let template = ProjectsTemplate {
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate {
    version: String,
}
