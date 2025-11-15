use actix_web::web;
use crate::endpoints::doi::doi_api;
use crate::endpoints::favicon::{favicon_ico, favicon_svg};
use crate::endpoints::index::{index, root_index};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root_index)
        .service(index)
        .service(favicon_ico)
        .service(favicon_svg)
        .service(
            web::scope("/v1")
                .service(doi_api)
        );
}