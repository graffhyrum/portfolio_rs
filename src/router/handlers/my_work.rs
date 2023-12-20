use askama::Template;
use axum::response::IntoResponse;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn my_work() -> impl IntoResponse {
    let template = MyWorkTemplate {
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/my_work/my_work.html")]
struct MyWorkTemplate {
    version: String,
}
