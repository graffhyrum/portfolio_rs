mod handlers;

use std::fs::File;
use std::io::Read;
use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use toml::Value;
use handlers::project;
use crate::router::handlers::{bookshelf::bookshelf, index::index};

pub fn build_router() -> Router {
    let assets_path_buff = std::env::current_dir().unwrap();
    let assets_path = assets_path_buff.to_str().unwrap();

    Router::new()
        .route("/", get(index))
        .route("/projects.html", get(project::projects))
        .route("/bookshelf.html", get(bookshelf))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path)))
        .nest_service("/styles", ServeDir::new(format!("{}/styles", assets_path)))
        .nest_service("/scripts", ServeDir::new(format!("{}/scripts", assets_path)))
        .nest_service("/favicon_io", ServeDir::new(format!("{}/favicon_io", assets_path)))
}

pub(crate) fn get_version() -> String {
    let mut contents = String::new();
    File::open("Cargo.toml").unwrap().read_to_string(&mut contents).unwrap();
    contents.parse::<Value>().unwrap()["package"]["version"].as_str().unwrap().to_string()
}