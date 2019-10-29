pub mod authenticator;
mod claims;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use showman_core::error::{Error, Result};
use showman_db::models::User;
use showman_db::models::role::Role;
use showman_db::schema::session::dsl as dsl_sess;
use showman_db::schema::user::dsl as dsl_user;
use showman_db::establish_connection;

use authenticator::Authenticator;
use claims::Claims;

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct Session {
    pub token: String,
    pub user_id: u32,
    pub expiration: NaiveDateTime
}

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct SessionData {
    pub(in crate) session: Session,
    pub(in crate) user: User
}

impl SessionData {
    pub fn role(&self) -> Role {
        self.user.role
    }

    pub fn user_id(&self) -> u32 {
        self.user.user_id
    }

    pub fn name(&self) -> &str {
        &self.user.name
    }
}

pub fn get(token: &str) -> Result<SessionData> {
    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    let session_data = dsl_sess::session
        .filter(dsl_sess::token.eq(token))
        .inner_join(dsl_user::user)
        .first::<SessionData>(&connection)
        .map_err(|e| if e == diesel::result::Error::NotFound {
            Error::Unauthorized()
        } else {
            Error::InternalServerError()
        })?;

    Ok(session_data)
}

pub fn create<A>(auth: A) -> Result<String>
    where
        A: Authenticator
{
    let user_id = auth.user_id();

    let claims = Claims::new(user_id);
    let token = claims.to_token()?;
    let now = Utc::now().timestamp();
    let exp = NaiveDateTime::from_timestamp(now + 7200, 0);

    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    diesel::insert_into(dsl_sess::session)
        .values((
            dsl_sess::token.eq(&token),
            dsl_sess::user_id.eq(user_id),
            dsl_sess::expiration.eq(exp)
        )).execute(&connection)
        .map_err(|_| Error::InternalServerError())?;

    Ok(token)
}

pub fn destroy(token: &str) -> Result<()> {
    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    diesel::delete(dsl_sess::session
        .filter(dsl_sess::token.eq(token))
    ).execute(&connection)
        .map_err(|_| Error::InternalServerError())?;

    Ok(())
}

pub fn update(token: &str) -> Result<()> {
    let now = Utc::now().timestamp();
    let exp = NaiveDateTime::from_timestamp(now + 7200, 0);

    let claims = Claims::from_token(token)?;

    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    connection.transaction(|| {

        let session: Session = dsl_sess::session
            .filter(dsl_sess::token.eq(token))
            .first(&connection)?;

        if claims.user_id() != session.user_id {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        diesel::update(dsl_sess::session
            .filter(dsl_sess::token.eq(token))
        ).set(dsl_sess::expiration.eq(exp))
            .execute(&connection)?;

        Ok(())
    }).map_err(|e| match e {
        diesel::result::Error::RollbackTransaction => Error::Unauthorized(),
        _ => Error::InternalServerError()
    })?;

    Ok(())
}