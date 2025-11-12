use actix_web::{get, web, HttpResponse, Responder};
use crate::utils::paper::Paper;

#[get("/doi/{tail:.*}")]
async fn doi_api(path: web::Path<String>) -> impl Responder {
    let doi = path.trim();

    if !(doi.starts_with("10.") && doi.contains('/')) {
        return HttpResponse::BadRequest()
            .body("Path must be a bare DOI, e.g., /v1/doi/10.xxx/yyy");
    }
    let pdl = Paper::new();

    match pdl.fetch_pdf_httpresponse(doi).await {
        Ok(resp) => resp,
        Err(e) => e.error_response(),
    }
}