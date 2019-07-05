  
use actix_web::{web};

use crate::service::index;

pub fn get(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("/{name}", web::get().to_async(index::handle))
    );
}