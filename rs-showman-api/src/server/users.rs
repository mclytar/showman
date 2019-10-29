pub mod user_id;

use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use validator::Validate;

use showman_core::error::Error;
use showman_db::{establish_connection, schema};
use diesel::prelude::*;

no_arg_sql_function!(last_insert_id, diesel::sql_types::Unsigned<diesel::sql_types::Integer>);

#[derive(Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(email)]
    pub username: String,
    #[validate(length(min = 5))]
    pub password: String,
    #[validate(must_match = "password")]
    pub password_confirm: String
}

#[derive(Serialize)]
pub struct UserId {
    pub user_id: u32
}

pub fn get(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub fn post(form: web::Form<RegisterForm>) -> HttpResponse {
    match form.validate() {
        Ok(_) => (),
        Err(e) => return Error::UnprocessableEntity().json(e).into()
    }

    let email = form.username.clone();
    let password = match bcrypt::hash(&form.password, 12) {
        Ok(password) => password,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let connection = match establish_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    match connection.transaction(|| {
        let user_id = {
            use schema::user::dsl;
            diesel::insert_into(dsl::user)
                .values((dsl::name.eq(""), dsl::surname.eq("")))
                .execute(&connection)?;

            diesel::select(last_insert_id)
                .first(&connection)?
        };

        use schema::authentication::dsl;
        diesel::insert_into(dsl::authentication)
            .values((dsl::user_id.eq(user_id), dsl::method.eq("password"), dsl::user_data.eq(&email), dsl::token.eq(&password)))
            .execute(&connection)?;

        Ok(user_id)
    }) {
        Ok(user_id) => HttpResponse::Created().json(UserId { user_id }),
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => HttpResponse::Conflict().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}