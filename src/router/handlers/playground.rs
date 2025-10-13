use askama::Template;
use axum::response::Html;

pub async fn playground() -> Html<String> {
    let template = PlaygroundTemplate {};
    Html(template.render().expect("Failed to render playground template"))
}

#[derive(Template)]
#[template(path = "pages/playground.html")]
struct PlaygroundTemplate {}
