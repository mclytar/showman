#[macro_use]
extern crate serde;

pub mod env;
pub mod pages;

use std::str::FromStr;

use actix_web::{HttpRequest};
use actix_web::web::{
    self,
    HttpResponse
};
use lazy_static::lazy_static;
use tera::Context;
use tera::Tera;

use pages::index::IndexContext;
use pages::show::ShowContext;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("www/pages/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    let _ = &TEMPLATES;
    cfg
        .route("", web::get().to(index))
        .route("/", web::get().to(index))
        .route("/index", web::get().to(index))
        .route("/shows/{id}", web::get().to(show))
        .route("/scenes/{id}/script", web::get().to(scene_script))
        .route("/{filename:\\w*\\.\\w*}", web::get().to(static_file))
        .route("/{filename:img/.*}", web::get().to(static_file))
    ;
}

async fn static_file(req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    let path: std::path::PathBuf = req
        .match_info()
        .query("filename")
        .parse()
        .unwrap();
    let path = std::path::PathBuf::from_str("www/static").unwrap().join(&path);
    let file = actix_files::NamedFile::open(path)?;
    Ok(file)
}

pub fn index(sc: IndexContext) -> HttpResponse {
    let context = if let Ok(context) = Context::from_serialize(sc) {
        context
    } else { return HttpResponse::InternalServerError().finish(); };

    match TEMPLATES.render("index.html", &context) {
        Ok(contents) => HttpResponse::Ok().body(contents),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub fn show(sc: ShowContext) -> HttpResponse {
    let context = if let Ok(context) = Context::from_serialize(sc) {
        context
    } else { return HttpResponse::InternalServerError().finish(); };

    match TEMPLATES.render("show.html", &context) {
        Ok(contents) => HttpResponse::Ok().body(contents),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub fn scene_script(sc: ShowContext) -> HttpResponse {
    let context = if let Ok(context) = Context::from_serialize(sc) {
        context
    } else { return HttpResponse::InternalServerError().finish(); };

    match TEMPLATES.render("script.html", &context) {
        Ok(contents) => HttpResponse::Ok().body(contents),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}