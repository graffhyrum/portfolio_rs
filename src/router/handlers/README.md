# Handlers Module

### [`index.rs`](index.rs), [`project.rs`](project.rs), and [`bookshelf.rs`](bookshelf.rs)

These files contain the handlers for the index, projects, and bookshelf routes respectively. They all use the `askama`
crate for templating and `axum` for response handling. Each file has an asynchronous
function (`index`, `projects`, `bookshelf`) that returns a type implementing `IntoResponse`. These functions create a
template instance (`IndexTemplate`, `ProjectsTemplate`, `BookshelfTemplate`) with a version.

The `BookshelfTemplate` also includes a vector of `ContentEntry` instances, each representing a book with a title, a
link (href), and a description.

### [`html_template.rs`](html_template.rs)

All three handler files use a shared `HtmlTemplate` struct from `html_template.rs` for their responses. The HTML templates are [located here](../../../templates).

Please note that this documentation is a high-level overview and does not cover all the details. For a more in-depth
understanding, it's recommended to read the source code directly.