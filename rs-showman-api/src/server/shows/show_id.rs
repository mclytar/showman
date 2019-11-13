use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use diesel::prelude::*;

use showman_db::establish_connection;
use showman_db::schema::show::dsl;
use showman_db::models::{Show, ShowData};
use showman_db::models::role::Role;

pub fn get(req: HttpRequest, path: web::Path<u32>) -> HttpResponse {
    let session_data = require_auth!(req);
    let show_id = path.into_inner();

    let role = session_data.role();

    if role == Role::Invalid || role == Role::Pending || role == Role::Banned {
        return HttpResponse::Forbidden().finish();
    }

    let connection = match establish_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let show: Show = match dsl::show.filter(dsl::show_id.eq(show_id))
        .first(&connection) {
        Ok(show) => show,
        Err(diesel::result::Error::NotFound) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let mut data = ShowData::from(show);

    if role == Role::Maintainer || role == Role::Administrator {
        data.grant();
    } else {
        data.restrict();
    }

    HttpResponse::Ok().json(data)
}

pub fn put(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub fn delete(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}