use actix_web::web;
use crate::endpoints::doi::doi_api;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(doi_api)
    );
}