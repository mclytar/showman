use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use showman_db::establish_connection;
use showman_db::schema::show::dsl;
use showman_db::models::{Show, ShowData};
use showman_db::models::role::Role;

no_arg_sql_function!(last_insert_id, diesel::sql_types::Unsigned<diesel::sql_types::Integer>);

#[derive(Deserialize, Validate)]
pub struct NewShowForm {
    #[validate(length(max = 256))]
    pub title: String,
    #[validate(length(max = 256))]
    pub subtitle: Option<String>,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    #[validate(length(max = 256))]
    pub notes: Option<String>
}

#[derive(Deserialize)]
pub struct ShowListParameters {
    pub items: Option<i64>,
    pub page: Option<i64>
}

#[derive(Serialize)]
pub struct ShowId {
    pub show_id: u32
}

pub fn get(req: HttpRequest, params: web::Query<ShowListParameters>) -> HttpResponse {
    let session_data = require_auth!(req);

    let role = session_data.role();

    if role == Role::Invalid || role == Role::Pending || role == Role::Banned {
        return HttpResponse::Forbidden().finish();
    }

    let connection = match establish_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let shows_result = match (params.items, params.page) {
        (Some(items), Some(page)) => dsl::show.limit(items).offset(items * (page - 1)).load(&connection),
        (Some(items), None) => dsl::show.limit(items).load(&connection),
        (None, Some(_)) => return HttpResponse::BadRequest().finish(),
        (None, None) => dsl::show.load(&connection)
    };

    let shows: Vec<Show> = match shows_result {
        Ok(shows) => shows,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let shows: Vec<ShowData> = shows.into_iter()
        .map(|s| {
            let mut data = ShowData::from(s);

            if role == Role::Maintainer || role == Role::Administrator {
                data.grant();
            } else {
                data.restrict();
            }

            data
        }).collect();

    HttpResponse::Ok().json(shows)
}

pub fn post(req: HttpRequest, form: web::Form<NewShowForm>) -> HttpResponse {
    let session_data = require_auth!(req);

    let new_show = form.into_inner();

    let role = session_data.role();

    if role != Role::Maintainer && role != Role::Administrator && role != Role::Organizer {
        return HttpResponse::Forbidden().finish();
    }

    let connection = match establish_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    match connection.transaction(|| {
        diesel::insert_into(dsl::show)
            .values((
                dsl::title.eq(&new_show.title),
                dsl::subtitle.eq(&new_show.subtitle),
                dsl::description.eq(&new_show.description),
                dsl::notes.eq(&new_show.notes)
            ))
            .execute(&connection)?;

        diesel::select(last_insert_id)
            .first(&connection)
    }) {
        Ok(show_id) => HttpResponse::Created().json(ShowId { show_id }),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}