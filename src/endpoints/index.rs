use actix_web::{get, HttpResponse, Responder};
use crate::config::constant::{SERVICE_NAME, SERVICE_DESCRIPTION, SERVICE_TITLE, HUB_NAME, HUB_SUBTITLE};
use crate::templates::generate_examples_html;

const INDEX_HTML: &str = include_str!("../html/index.html");
const DOI_HTML: &str = include_str!("../html/doi.html");

#[get("/")]
pub async fn root_index() -> impl Responder {
    let html = INDEX_HTML
        .replace("{{HUB_NAME}}", HUB_NAME)
        .replace("{{HUB_SUBTITLE}}", HUB_SUBTITLE);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/doi")]
pub async fn index() -> impl Responder {
    let examples_html = generate_examples_html();

    let html = DOI_HTML
        .replace("{{SERVICE_NAME}}", SERVICE_NAME)
        .replace("{{SERVICE_TITLE}}", SERVICE_TITLE)
        .replace("{{SERVICE_DESCRIPTION}}", SERVICE_DESCRIPTION)
        .replace("{{EXAMPLE_DOIS}}", &examples_html)
        .replace("{{HUB_NAME}}", HUB_NAME);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}