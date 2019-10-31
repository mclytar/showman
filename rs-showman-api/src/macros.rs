#[macro_export]
macro_rules! require_auth {
    ($req:expr) => {{
        let auth_token = match $req.cookie("auth") {
            Some(cookie) => cookie.value().to_owned(),
            None => return actix_web::HttpResponse::Unauthorized().finish()
        };
        match showman_auth_base::session::update(&auth_token) {
            Ok(_) => {},
            Err(_) => return actix_web::HttpResponse::Unauthorized().finish()
        }
        match showman_auth_base::session::get(&auth_token) {
            Ok(auth) => auth,
            Err(_) => return actix_web::HttpResponse::Unauthorized().finish()
        }
    }}
}