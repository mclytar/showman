mod session;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/session", web::post().to(session::post))
        .route("/session", web::delete().to(session::delete));
}