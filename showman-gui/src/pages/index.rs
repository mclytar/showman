use actix_web::{FromRequest, HttpRequest, Error, HttpMessage};
use actix_web::web::{
    Data
};
use actix_web::error::{ErrorServiceUnavailable, ErrorInternalServerError};
use actix_web::dev::{PayloadStream, Payload};
use futures::future::{
    self,
    Ready
};

use showman_data::prelude::*;

#[derive(Serialize)]
pub struct IndexContext {
    pub env: crate::env::Environment,
    pub user: Option<String>,
    pub shows: Vec<ShowData>
}

impl FromRequest for IndexContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload<PayloadStream>) -> Self::Future {
        let dbp = Data::<DbPool>::from_request(req, payload)
            .into_inner();

        let dbc = match dbp {
            Ok(dbp) => match dbp.get() {
                Ok(dbc) => dbc,
                Err(e) => return future::err(ErrorServiceUnavailable(e.to_string()))
            },
            Err(e) => return future::err(e)
        };

        let env = match crate::env::Environment::new() {
            Ok(env) => env,
            Err(e) => return future::err(ErrorInternalServerError(e.to_string()))
        };
        let user = if let Some(user) = req.cookie("user") {
            Some(user.value().to_owned())
        } else { None };

        let shows = match ShowData::load_all(&dbc) {
            Ok(shows) => shows,
            Err(e) => return future::err(e)
        };

        futures::future::ok(IndexContext { env, user, shows })
    }
}