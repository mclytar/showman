mod shows;
mod users;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/shows", web::get().to(shows::get))
        .route("/shows", web::post().to(shows::post))
        .route("/shows/{show_id}", web::get().to(shows::show_id::get))
        .route("/shows/{show_id}", web::get().to(shows::show_id::put))
        .route("/shows/{show_id}", web::get().to(shows::show_id::delete))
        .route("/users", web::post().to(users::post))
        .route("/users/{user_id}", web::get().to(users::user_id::get))
        .route("/users/{user_id}", web::put().to(users::user_id::put))
        .route("/users/{user_id}", web::delete().to(users::user_id::delete));
}