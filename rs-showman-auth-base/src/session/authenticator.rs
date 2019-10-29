use diesel::prelude::*;

use showman_core::error::{Error, Result};
use showman_db::models::Authentication;
use showman_db::schema::authentication::dsl as dsl_auth;
use showman_db::establish_connection;

pub trait Authenticator {
    fn user_id(&self) -> u32;
}

pub struct PasswordAuthenticator {
    user_id: u32
}

impl PasswordAuthenticator {
    pub fn from_credentials(username: &str, password: &str) -> Result<PasswordAuthenticator> {
        let connection = establish_connection()
            .map_err(|_| Error::InternalServerError())?;

        let auth: Authentication = dsl_auth::authentication
            .filter(dsl_auth::method.eq("password")
                    .and(
                    dsl_auth::user_data.eq(username)
                ))
            .first(&connection)
            .map_err(|e| if e == diesel::result::Error::NotFound {
                Error::Unauthorized()
            } else {
                Error::InternalServerError()
            })?;

        let user_id = auth.user_id;

        if !bcrypt::verify(password, &auth.token).map_err(|_| Error::InternalServerError())? {
            return Err(Error::Unauthorized().finish());
        }

        Ok(PasswordAuthenticator { user_id })
    }
}

impl Authenticator for PasswordAuthenticator {
    fn user_id(&self) -> u32 {
        self.user_id
    }
}