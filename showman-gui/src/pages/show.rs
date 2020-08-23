use std::str::FromStr;

use actix_web::{FromRequest, HttpRequest, Error, HttpMessage};
use actix_web::web::{
    Data
};
use actix_web::error::{ErrorInternalServerError};
use actix_web::dev::{PayloadStream, Payload};
use futures::future::{
    self,
    Ready
};

use showman_data::prelude::*;

#[derive(Serialize)]
pub struct ShowContext {
    pub env: crate::env::Environment,
    pub user: Option<String>,
    pub show: ShowData
}

impl FromRequest for ShowContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
        let env = match crate::env::Environment::new() {
            Ok(env) => env,
            Err(e) => return future::err(ErrorInternalServerError(e.to_string()))
        };
        let user = if let Some(user) = req.cookie("user") {
            Some(user.value().to_owned())
        } else { None };
        let show = ShowData {
            show_id: 1,
            title: "Test show".to_string(),
            description: Some("Show for test purposes.".to_owned()),
            creation: FromStr::from_str("2020-05-08T23:20:04").unwrap()
        };
        futures::future::ok(ShowContext { env, user, show })
    }
}