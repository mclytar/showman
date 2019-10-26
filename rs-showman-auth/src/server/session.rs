use actix_web::{web, HttpMessage, HttpResponse, HttpRequest};
use serde::{Deserialize};

use showman_auth_base::{session, token};
use showman_core::{authentication_cookie, get_navigation_auxiliary_data};

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

pub fn post(req: HttpRequest, form: web::Form<LoginForm>) -> HttpResponse {
    let aux = get_navigation_auxiliary_data(&req);

    match token::with_login(&form.username, &form.password, &aux) {
        Ok(auth) => HttpResponse::Created()
            .cookie(authentication_cookie(&auth))
            .finish(),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

pub fn delete(req: HttpRequest) -> HttpResponse {
    match cookie_get_auth!(req) {
        Some(claims) => {
            if let Err(_) = session::invalidate(claims.token_id()) {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::NoContent()
                .del_cookie(&req.cookie("auth").unwrap())
                .finish()
        },
        None => HttpResponse::NoContent().finish()
    }
}