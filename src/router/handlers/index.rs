use axum::response::IntoResponse;
use askama::Template;
use crate::router::version::get_version;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        version: get_version(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    version: String,
}
