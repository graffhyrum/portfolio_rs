use askama::Template;
use axum::response::IntoResponse;
use crate::router;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn projects() -> impl IntoResponse {
    let template = ProjectsTemplate {
        version: router::get_version(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "../templates/projects.html")]
struct ProjectsTemplate {
    version: String,
}
