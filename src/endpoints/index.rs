use actix_web::{get, HttpResponse, Responder};
use crate::config::constant::{SERVICE_NAME, SERVICE_DESCRIPTION, SERVICE_TITLE, HUB_NAME, HUB_SUBTITLE};
use crate::templates::generate_examples_html;

const BASE_HTML: &str = include_str!("../html/base.html");
const INDEX_CONTENT: &str = include_str!("../html/index_content.html");
const DOI_CONTENT: &str = include_str!("../html/doi_content.html");

fn build_page(content: &str, title: &str, nav_items: &str) -> String {
    // Extract sections from content
    let mut page_meta = "";
    let mut page_styles = "";
    let mut page_content = "";
    let mut page_scripts = "";

    // Parse content to extract different sections
    let sections: Vec<&str> = content.split("<!-- PAGE_").collect();
    for section in sections.iter().skip(1) {
        if section.starts_with("META -->") {
            page_meta = section.trim_start_matches("META -->").split("<!-- PAGE_").next().unwrap_or("").trim();
        } else if section.starts_with("STYLES -->") {
            page_styles = section.trim_start_matches("STYLES -->").split("<!-- PAGE_").next().unwrap_or("").trim();
        } else if section.starts_with("CONTENT -->") {
            page_content = section.trim_start_matches("CONTENT -->").split("<!-- PAGE_").next().unwrap_or("").trim();
        } else if section.starts_with("SCRIPTS -->") {
            page_scripts = section.trim_start_matches("SCRIPTS -->").trim();
        }
    }

    BASE_HTML
        .replace("{{PAGE_TITLE}}", title)
        .replace("{{PAGE_META}}", page_meta)
        .replace("{{PAGE_STYLES}}", page_styles)
        .replace("{{PAGE_CONTENT}}", page_content)
        .replace("{{PAGE_SCRIPTS}}", page_scripts)
        .replace("{{NAV_ITEMS}}", nav_items)
}

#[get("/")]
pub async fn root_index() -> impl Responder {
    let html = build_page(
        INDEX_CONTENT,
        &format!("{}", HUB_NAME),
        r#"<li><a href="/doi">DOIer</a></li>"#
    )
    .replace("{{HUB_NAME}}", HUB_NAME)
    .replace("{{HUB_SUBTITLE}}", HUB_SUBTITLE);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/doi")]
pub async fn index() -> impl Responder {
    let examples_html = generate_examples_html();

    let html = build_page(
        DOI_CONTENT,
        &format!("{} - {}", SERVICE_NAME, SERVICE_TITLE),
        r##"<li><a href="#api" onclick="return handleNavClick(event, 'api')">API</a></li>"##
    )
    .replace("{{SERVICE_NAME}}", SERVICE_NAME)
    .replace("{{SERVICE_TITLE}}", SERVICE_TITLE)
    .replace("{{SERVICE_DESCRIPTION}}", SERVICE_DESCRIPTION)
    .replace("{{EXAMPLE_DOIS}}", &examples_html)
    .replace("{{HUB_NAME}}", HUB_NAME);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}