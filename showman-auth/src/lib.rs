pub(crate) mod self_prelude {
    pub use actix_web::{
        web::{
            self,
        },
        HttpResponse
    };
}

use self_prelude::*;

pub fn setup(cfg: &mut web::ServiceConfig) {
    // TODO
    cfg
        .route("/session", web::post().to(log_in))
        .route("/session", web::delete().to(log_out))
    ;
}

fn log_in() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

fn log_out() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}