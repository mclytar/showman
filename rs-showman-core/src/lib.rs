pub mod error;

use actix_web::HttpRequest;
use actix_web::cookie::{Cookie, SameSite};
use hex::encode;
use ring::digest::{digest, SHA256};

pub fn authentication_cookie(token: &str) -> Cookie {
    Cookie::build("auth", token.to_owned())
        .path("/")
        .same_site(SameSite::Strict)
        //.secure(true)
        .http_only(true)
        .finish()
}

pub fn get_navigation_auxiliary_data(req: &HttpRequest) -> String {
    let aux = req
        .headers()
        .get("User-Agent")
        .map(|ua| ua.to_str())
        .and_then(|ua| ua.ok())
        .unwrap_or("unknown");

    encode(digest(&SHA256, aux.as_bytes()))
}