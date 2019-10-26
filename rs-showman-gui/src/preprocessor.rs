#![allow(non_upper_case_globals)]

pub mod settings;
mod template;
mod token;

use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use template::Template;
use settings::Settings;
use actix_web::HttpResponse;
use actix_web::dev::HttpResponseBuilder;

pub mod err {
    use actix_web::HttpResponse;
    use super::preprocessor;
    use super::settings::Settings;

    pub fn unauthorized(settings: &Settings) -> Result<HttpResponse, std::io::Error> {
        preprocessor.load("./www/error/401.html", settings).map(|s| HttpResponse::Unauthorized().body(s))
    }

    pub fn forbidden(settings: &Settings) -> Result<HttpResponse, std::io::Error> {
        preprocessor.load("./www/error/403.html", settings).map(|s| HttpResponse::Forbidden().body(s))
    }

    pub fn not_found(settings: &Settings) -> Result<HttpResponse, std::io::Error> {
        preprocessor.load("./www/error/404.html", settings).map(|s| HttpResponse::NotFound().body(s))
    }

    pub fn internal_server_error(settings: &Settings) -> Result<HttpResponse, std::io::Error> {
        preprocessor.load("./www/error/500.html", settings).map(|s| HttpResponse::NotFound().body(s))
    }
}

pub mod prelude {
    pub use super::settings::Settings;
    pub mod preprocessor {
        pub use super::super::{update, load, get_template, err};
    }
}

lazy_static!{
    static ref preprocessor: Preprocessor = Preprocessor::construct();
}

pub fn update<P>(templates_dir: P)
    where
        P: AsRef<Path>
{
    preprocessor.update(templates_dir);
}

pub fn parse(contents: &str, settings: &Settings) -> String {
    preprocessor.parse(contents, settings)
}

pub fn load<P>(filename: P, settings: &Settings) -> Result<String, std::io::Error>
    where
        P: AsRef<Path>
{
    preprocessor.load(filename, settings)
}

pub fn load_with_builder<P, F>(filename: P, settings: &Settings, builder: F) -> Result<HttpResponse, std::io::Error>
    where
        P: AsRef<Path>,
        F: FnOnce(HttpResponseBuilder) -> HttpResponseBuilder
{
    preprocessor.load(filename, settings).map(|s| builder(HttpResponse::Ok()).body(s))
}

pub fn get_template(name: &str, settings: &Settings) -> Option<String> {
    preprocessor.template(name, settings)
}

struct Preprocessor {
    templates: RwLock<HashMap<String, Template>>
}

impl Preprocessor {
    pub fn construct() -> Preprocessor {
        Preprocessor {
            templates: RwLock::new(HashMap::new())
        }
    }

    pub fn update<P>(&self, templates_dir: P)
        where
            P: AsRef<Path>
    {
        let mut templates = self.templates.write().unwrap();
        templates.clear();

        let templates_dir = std::fs::read_dir(templates_dir).unwrap()
            .filter(|dir| dir.is_ok())
            .map(|dir| dir.unwrap())
            .filter(|dir| dir.path().is_dir());

        for dir in templates_dir {
            let name = dir.file_name().to_string_lossy().to_string();
            let template_dir = std::fs::read_dir(dir.path()).unwrap()
                .filter(|dir| dir.is_ok())
                .map(|dir| dir.unwrap())
                .filter(|dir| dir.path().is_file());
            let mut template = Template::new();
            for file in template_dir {
                let template_name = file.file_name().to_string_lossy().replace(".html", "");
                template.set(&template_name, file.path()).unwrap();
            }
            templates.insert(name, template);
        }
    }

    pub fn parse(&self, contents: &str, settings: &Settings) -> String {
        let regex = Regex::new(r#"<\?rs (\w+)::(.*?) \?>"#).unwrap();

        regex.replace_all(&contents, |captures: &Captures| {
            let result = match &captures[1] {
                "var" => settings.var(&captures[2]).map(|v| v.to_string()),
                "env" => std::env::var(&captures[2]).ok(),
                "template" => self.template(&captures[2], settings),
                _ => { None }
            };
            result.unwrap_or(String::new())
        }).into_owned()
    }

    pub fn load<P>(&self, filename: P, settings: &Settings) -> Result<String, std::io::Error>
        where
            P: AsRef<Path>
    {
        std::fs::read_to_string(filename).map(|c| self.parse(&c, settings))
    }

    pub fn template(&self, name: &str, settings: &Settings) -> Option<String> {
        if let Ok(guard) = self.templates.read() {
            guard.get(name).map(|t| t.get(name, settings))
        } else {
            None
        }
    }
}