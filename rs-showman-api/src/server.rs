mod users;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/users", web::get().to(users::get))
        .route("/users", web::post().to(users::post))
        .route("/users/{user_id}", web::get().to(users::user_id::get))
        .route("/users/{user_id}", web::put().to(users::user_id::put))
        .route("/users/{user_id}", web::delete().to(users::user_id::delete));
}