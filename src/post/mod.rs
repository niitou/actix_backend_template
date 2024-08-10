mod controller;
mod dto;
mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .service(controller::add_post)
            .service(controller::delete_post)
            .service(controller::update_post)
            .service(controller::get_post)
            .service(controller::get_post_all),
    );
}
