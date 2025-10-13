use askama::Template;
use axum::response::Html;

pub async fn index() -> Html<String> {
    let template = IndexTemplate {};
    Html(template.render().expect("Failed to render index template"))
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {}
