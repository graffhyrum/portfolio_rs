use axum::response::IntoResponse;
use askama::Template;
use crate::router;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        version: router::get_version(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "../templates/index.html")]
struct IndexTemplate {
    version: String,
}
