use askama::Template;
use axum::response::IntoResponse;
use crate::router::version::get_version;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn projects() -> impl IntoResponse {
    let template = ProjectsTemplate {
        version: get_version(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate {
    version: String,
}
