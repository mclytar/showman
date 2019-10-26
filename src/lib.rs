pub mod cli;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate showman_cli;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::dev::Server;

use showman_api::server::configure as api_configure;
use showman_auth::server::configure as auth_configure;
use showman_gui::server::configure as gui_configure;
use showman_gui::server::not_found;

lazy_static! {
    static ref SERVER: Mutex<Option<Server>> = Mutex::new(None);
}

pub fn is_started() -> bool {
    SERVER.lock().unwrap().is_some()
}

pub fn stop(graceful: bool) {
    let mut server_guard = SERVER.lock().unwrap();
    if let Some(http_server) = server_guard.take() {
        let _ = http_server.stop(graceful);
    }
}

pub fn start() {
    let http_server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .configure(api_configure)
            )
            .service(
                web::scope("/www")
                    .configure(gui_configure)
            )
            .service(
                web::scope("/auth")
                    .configure(auth_configure)
            )
            .service(
                web::scope("/")
                    .route("/", web::to(|| HttpResponse::PermanentRedirect().set_header("Location", "/www/index").finish()))
                    .default_service(web::to(|req: HttpRequest| {
                        let uri = req.path();

                        let new_uri = if uri.starts_with("/") {
                            "/www".to_owned() + uri
                        } else {
                            "/www/".to_owned() + uri
                        };

                        HttpResponse::PermanentRedirect().set_header("Location", new_uri).finish()
                    }))
            )
            .default_service(web::to(not_found))
    }).bind("0.0.0.0:8000")
        .expect("Cannot bind to port 8000.")
        .start();

    let mut server_guard = SERVER.lock().unwrap();

    *server_guard = Some(http_server);
}