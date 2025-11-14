use actix_web::web;
use crate::endpoints::doi::doi_api;
use crate::endpoints::index::{index, root_redirect};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root_redirect)
        .service(index)
        .service(
            web::scope("/v1")
                .service(doi_api)
        );
}