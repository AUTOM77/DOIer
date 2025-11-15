use actix_web::web;
use crate::endpoints::doi::doi_api;
use crate::endpoints::favicon::favicon_ico;
use crate::endpoints::index::{index, root_index};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root_index)
        .service(index)
        .service(favicon_ico)
        .service(
            web::scope("/v1")
                .service(doi_api)
        );
}