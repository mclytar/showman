pub mod cli;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate showman_cli;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::dev::{Server, Service};

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

pub fn get_tls_configuration() -> rustls::ServerConfig {
    let mut tls_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    let cert_file = std::fs::read("cert.pem")
        .expect("Cannot open certificate file.");
    let key_file = std::fs::read("key.pem")
        .expect("Cannot open key file.");

    let cert_chain = rustls::internal::pemfile::certs(&mut &cert_file[..])
        .expect("Invalid certificate file.");
    let key = rustls::internal::pemfile::pkcs8_private_keys(&mut &key_file[..])
        .expect("Invalid key file.")
        .remove(0);

    tls_config.set_single_cert(cert_chain, key).unwrap();

    tls_config
}

pub fn start() {
    let bind = std::env::var("BIND")
        .unwrap_or("127.0.0.1:80".to_owned());
    let bind_ssl = std::env::var("BIND_SSL")
        .unwrap_or("127.0.0.1:443".to_owned());

    let http_server = HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                // Redirect HTTP to HTTPS
                if req.connection_info().scheme() != "https" {
                    let port = std::env::var("BIND_SSL")
                        .ok()
                        .and_then(|s| s.split(":").last().map(|s| s.to_owned()))
                        .unwrap_or("".to_owned());
                    let host = req.connection_info()
                        .host()
                        .split(":")
                        .next()
                        .unwrap()
                        .to_owned();
                    let resource = req.uri()
                        .path_and_query()
                        .map(|p| p.as_str())
                        .unwrap_or("/");
                    let redirect_uri = format!("https://{}:{}{}", &host, &port, resource);

                    futures::future::Either::A(futures::future::ok(
                        req.into_response(
                            HttpResponse::PermanentRedirect().set_header("Location", redirect_uri).finish()
                        )
                    ))
                } else {
                    srv.call(req)
                }
            })
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
    }).bind(&bind)
        .expect(&format!("Cannot bind to {}.", &bind))
        .bind_rustls(&bind_ssl, get_tls_configuration())
        .expect(&format!("Cannot secure bind to {}.", &bind_ssl))
        .start();

    let mut server_guard = SERVER.lock().unwrap();

    *server_guard = Some(http_server);
}