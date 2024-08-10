mod controller;
mod dto;
mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comment")
            .service(controller::add_comment)
            .service(controller::delete_comment)
            .service(controller::update_comment)
            .service(controller::get_comment),
    );
}
