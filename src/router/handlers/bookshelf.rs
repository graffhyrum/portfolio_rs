use askama::Template;
use axum::response::IntoResponse;
use crate::router::version::get_version;
use crate::router::handlers::html_template::HtmlTemplate;

pub async fn bookshelf() -> impl IntoResponse {
            let template = BookshelfTemplate {
        version: get_version(),
                entries: vec![
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
                    ContentEntry {
                        title: "The Twelve Factor App",
                        href: "https://12factor.net/",
                        description: "A methodology for building modern, scalable, maintainable software-as-a-service apps.",
            }
                ],
            };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "bookshelf.html")]
struct BookshelfTemplate<'a> {
    version: String,
    entries: Vec<ContentEntry<'a>>,
}

struct ContentEntry<'a> {
    title: &'a str,
    href: &'a str,
    description: &'a str,
}
