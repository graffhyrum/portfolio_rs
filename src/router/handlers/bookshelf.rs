use askama::Template;
use axum::response::IntoResponse;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn bookshelf() -> impl IntoResponse {
    let template = BookshelfTemplate {
        version: env!("CARGO_PKG_VERSION").to_string(),
        disciplines: vec![
            ContentEntry {
                title: "The Twelve Factor App",
                href: "https://12factor.net/",
                description: "A methodology for building modern, scalable, maintainable software-as-a-service apps.",
            },
        ],
        articles: vec![
            ContentEntry {
                title: "The Grug Brained Developer",
                href: "https://grugbrain.dev/",
                description: "A layman's guide to thinking like the self-aware smol brained.",
            },
            ContentEntry {
                title: "HTMX on Locality of Behavior",
                href: "https://htmx.org/essays/locality-of-behaviour/",
                description: "An article on the tradeoffs of Separation of Concerns and Locality of Behavior.",
            },
        ],
    };
    HtmlTemplate(template)
}


//region Structs
#[derive(Template)]
#[template(path = "bookshelf.html")]
struct BookshelfTemplate<'a> {
    version: String,
    disciplines: Vec<ContentEntry<'a>>,
    articles: Vec<ContentEntry<'a>>,
}

struct ContentEntry<'a> {
    title: &'a str,
    href: &'a str,
    description: &'a str,
}
//endregion