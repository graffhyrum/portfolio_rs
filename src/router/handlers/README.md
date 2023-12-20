# Handlers Module

### [`index.rs`](index.rs), [`my_work.rs`](my_work.rs), and [`bookshelf.rs`](bookshelf.rs)

These files contain the handlers for the index, projects, and bookshelf routes respectively. They all use the `askama`
crate for templating and `axum` for response handling. Each file has an asynchronous
function (`index`, `projects`, `bookshelf`) that returns a type implementing `IntoResponse`. These functions create a
template instance (`IndexTemplate`, `ProjectsTemplate`, `BookshelfTemplate`) with a version.

The `BookshelfTemplate` also includes a vector of `ContentEntry` instances, each representing a book with a title, a
link (href), and a description.
