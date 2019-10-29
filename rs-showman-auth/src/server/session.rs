use actix_web::{web, HttpMessage, HttpResponse, HttpRequest};
use serde::{Deserialize};

use showman_auth_base::session;
use showman_auth_base::session::authenticator::PasswordAuthenticator;
use showman_core::authentication_cookie;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

pub fn post(form: web::Form<LoginForm>) -> HttpResponse {
    let auth = match PasswordAuthenticator::from_credentials(&form.username, &form.password) {
        Ok(auth) => auth,
        Err(e) => return e.into()
    };

    let token = match session::create(auth) {
        Ok(token) => token,
        Err(e) => return e.into()
    };

    HttpResponse::Created()
        .cookie(authentication_cookie(&token))
        .finish()
}

pub fn delete(req: HttpRequest) -> HttpResponse {
    let token = match req.cookie("auth") {
        Some(cookie) => cookie.value().to_owned(),
        None => return HttpResponse::NoContent().finish()
    };

    match session::destroy(&token) {
        Ok(_) => {
            HttpResponse::NoContent()
                .del_cookie(&req.cookie("auth").unwrap())
                .finish()
        },
        Err(e) => e.into()
    }
}