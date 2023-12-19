use std::fs::File;
use std::io::Read;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use axum::response::{Html, IntoResponse, Response};
use askama::Template;
use axum::http::StatusCode;
use toml::Value;


pub fn build_router() -> Router {
    let assets_path_buff = std::env::current_dir().unwrap();
    let assets_path = assets_path_buff.to_str().unwrap();

    let router = Router::new()
        .route("/", get(index))
        .route("/projects.html", get(projects))
        .route("/bookshelf.html", get(bookshelf))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path),
            ))
        .nest_service(
            "/styles",
            ServeDir::new(format!("{}/styles", assets_path)))
        .nest_service(
            "/scripts",
            ServeDir::new(format!("{}/scripts", assets_path),
            )).nest_service(
        "/favicon_io",
        ServeDir::new(format!("{}/favicon_io", assets_path
        )),
    );

    router
}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        version: get_version(),
    };
    HtmlTemplate(template)
}


pub async fn projects() -> impl IntoResponse {
    let template = ProjectsTemplate {
        version: get_version(),
    };
    HtmlTemplate(template)
}

pub async fn bookshelf() -> impl IntoResponse {
    let template = BookshelfTemplate {
        version: get_version(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "../templates/index.html")]
struct IndexTemplate {
    version: String,
}

#[derive(Template)]
#[template(path = "../templates/projects.html")]
struct ProjectsTemplate {
    version: String,
}

#[derive(Template)]
#[template(path = "../templates/bookshelf.html")]
struct BookshelfTemplate {
    version: String,
}


struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template, Error: {}", err),
            )
                .into_response(),
        }
    }
}


fn get_version() -> String {
    // Read the Cargo.toml file
    let mut file = File::open("Cargo.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the TOML
    let value = contents.parse::<Value>().unwrap();

    // Get the version
    let version = value["package"]["version"].as_str().unwrap();

    version.to_string()
}