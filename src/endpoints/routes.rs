use actix_web::web;
use crate::endpoints::doi::doi_api;
use crate::endpoints::index::index;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(
            web::scope("/v1")
                .service(doi_api)
        );
}