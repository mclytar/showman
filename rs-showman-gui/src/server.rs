use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};

use showman_db::models::role::Role;

use crate::preprocessor;
use crate::preprocessor::settings::Settings;

pub fn not_found(req: HttpRequest) -> Result<HttpResponse> {
    let mut settings = Settings::new();

    if let Some(claims) = cookie_get_auth!(req) {
        settings.set_var("username", claims.display());
        settings.set_var("user_id", &format!("{}", claims.user_id()));
        settings.set_template("appbar", if claims.role() == Role::Maintainer || claims.role() == Role::Administrator { "admin" } else { "user" });
    }

    Ok(preprocessor::err::not_found(&settings)?)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/",
              web_page!("index.html"))
        .route("/index",
               web_page!("index.html"))
        .route("/login",
               web_page!("login.html"))
        .route("/register",
               web_page!("register.html"))
        .route("/shows",
               reserved_web_page!("shows.html"))
        .route("/shows/{show_id}",
               reserved_web_page!("shows/{show_id}.html", show_id:u32))
        .route("/shows/{show_id}/{scene_id}",
               reserved_web_page!("shows/{show_id}/{scene_id}.html", show_id:u32, scene_id:u32))
        .route("/users",
               reserved_web_page!("users.html", (req, claims, settings) => {
                    if claims.role() != Role::Maintainer && claims.role() != Role::Administrator {
                        return preprocessor::err::forbidden(&settings);
                    }
               }))
        .route("/users/{id}",
               reserved_web_page!("users/{user_id}.html", id:u32, (req, claims, settings) => {
                    if claims.role() != Role::Maintainer && claims.role() != Role::Administrator && claims.user_id() != id {
                        return preprocessor::err::forbidden(&settings);
                    }
               }))
        .service(actix_files::Files::new("/", "./www/static/"));
        /*.service(web::scope("/")
            .default_service(web::to(|req: actix_web::HttpRequest| {
                let mut settings = Settings::new();

                if let Some(claims) = cookie_get_auth!(req) {
                    settings.set_var("username", claims.display());
                    settings.set_var("user_id", &format!("{}", claims.user_id()));
                    settings.set_template("appbar", if claims.role() == Role::Maintainer || claims.role() == Role::Administrator { "admin" } else { "user" });
                }

                preprocessor::err::not_found(&settings)
            }))
        );*/
}