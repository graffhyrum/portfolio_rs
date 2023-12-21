mod handlers;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use crate::router::handlers::{
    bookshelf::bookshelf,
    index::index,
    my_work::my_work,
    testimonials::testimonial,
    playground::playground};
use anyhow::Result;
use axum::http::StatusCode;

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}


pub fn build_router() -> Result<Router> {
    let assets_path_buff = std::env::current_dir()?;
    let assets_path = assets_path_buff
        .to_str()
        .ok_or(anyhow::anyhow!("Failed to convert path to string"))?;

    let routes = Router::new()
        .route("/", get(index))
        .route("/my_work.html", get(my_work))
        .route("/bookshelf.html", get(bookshelf))
        .route("/testimonials.html", get(testimonial))
        .route("/playground.html", get(playground))
        .fallback(fallback);

    let services = Router::new()
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path)))
        .nest_service("/styles", ServeDir::new(format!("{}/styles", assets_path)))
        .nest_service("/scripts", ServeDir::new(format!("{}/scripts", assets_path)))
        .nest_service("/favicon_io", ServeDir::new(format!("{}/favicon_io", assets_path)));

    let app = Router::new()
        .merge(routes)
        .merge(services);


    Ok(app)
}
