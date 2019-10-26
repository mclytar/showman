use bcrypt::{hash, verify};
use chrono::Utc;
use diesel::prelude::*;

use showman_core::error::{Error, Result};
use showman_db::{establish_connection, schema};
use showman_db::models::{Authentication, User};
use crate::claims::Claims;
use crate::session;
use crate::session::SessionStatus;

pub fn with_login(username: &str, password: &str, aux: &str) -> Result<String> {
    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    let authentication = {
        use schema::authentication::dsl;

        let result: Authentication = dsl::authentication
            .filter(
                dsl::method.eq("password")
                    .and(dsl::user_data.eq(username))
            )
            .first(&connection)
            .map_err(|e| if e == diesel::result::Error::NotFound {
                let _dummy = hash("dummy", 12);
                Error::Unauthorized()
            } else {
                Error::InternalServerError()
            })?;

        result
    };

    if !verify(password, &authentication.token).map_err(|_| Error::InternalServerError())? {
        return Err(Error::Unauthorized().finish());
    }

    let timestamp = Utc::now().timestamp();
    let token = session::create(authentication.user_id, timestamp + 300)?;

    let user: User = {
        use schema::user::dsl;

        let result = dsl::user
            .filter(dsl::user_id.eq(authentication.user_id))
            .first(&connection)
            .map_err(|_| Error::InternalServerError())?;

        result
    };

    let claims = Claims::from_auth(user, token, aux);

    claims.token()
}

pub fn refresh(old: &str, aux: &str) -> Result<String> {
    let claims = Claims::from_token(old, aux, &crate::SIGN_KEY)?;

    match session::status(claims.token_id())? {
        SessionStatus::Valid => Ok(old.to_owned()),
        SessionStatus::Invalid => Err(Error::Unauthorized().finish()),
        SessionStatus::NeedsUpdate => {
            let timestamp = Utc::now().timestamp();
            let token = session::create(claims.user_id(), timestamp + 300)?;

            let connection = establish_connection()
                .map_err(|_| Error::InternalServerError())?;

            let user: User = {
                use schema::user::dsl;

                let result = dsl::user
                    .filter(dsl::user_id.eq(claims.user_id()))
                    .first(&connection)
                    .map_err(|_| Error::InternalServerError())?;

                result
            };

            session::invalidate(claims.token_id())?;

            let claims = Claims::from_auth(user, token, aux);
            claims.token()
        },
        SessionStatus::Expired => {
            let timestamp = Utc::now().timestamp();
            let token = session::create(claims.user_id(), timestamp + 300)?;

            session::invalidate(claims.token_id())?;

            let claims = claims.update(timestamp, token);
            claims.token()
        }
    }
}