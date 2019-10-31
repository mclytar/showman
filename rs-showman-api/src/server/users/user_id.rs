use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use diesel::prelude::*;

use showman_db::establish_connection;
use showman_db::models::User;
use showman_db::models::role::Role;

pub fn get(req: HttpRequest, path: web::Path<u32>) -> HttpResponse {
    let session_data = require_auth!(req);

    let req_user_id = path.into_inner();

    let role = session_data.role();
    let user_id = session_data.user_id();

    if role != Role::Maintainer && role != Role::Administrator && user_id != req_user_id {
        return HttpResponse::Forbidden().finish();
    }

    let connection = match establish_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let user_data: User = {
        use showman_db::schema::user::dsl;

        match dsl::user.filter(dsl::user_id.eq(req_user_id))
            .first(&connection) {
            Ok(user_data) => user_data,
            Err(diesel::result::Error::NotFound) => return HttpResponse::NotFound().finish(),
            Err(_) => return HttpResponse::InternalServerError().finish()
        }
    };

    HttpResponse::Ok().json(user_data)
}

pub fn put(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub fn delete(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}