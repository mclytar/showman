use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use showman_core::error::{Error, Result};
use showman_db::schema::session::dsl;
use showman_db::establish_connection;

use showman_db::schema::session as table_name_session;

no_arg_sql_function!(last_insert_id, diesel::sql_types::Unsigned<diesel::sql_types::Bigint>);

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct Session {
    pub token_id: u32,
    pub user_id: u32,
    pub needs_update: bool,
    pub expiration: NaiveDateTime
}

#[derive(Clone, Debug, Insertable, Deserialize)]
#[table_name = "table_name_session"]
struct NewSession {
    pub user_id: u32,
    pub expiration: NaiveDateTime
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Represents a session status for a token.
pub enum SessionStatus {
    /// Represents a valid token id.
    Valid,
    /// Represents a token id for a valid token that needs an update.
    NeedsUpdate,
    /// Represents an invalid token id for whatever reason.
    /// When a token is `Invalid`, the user needs to repeat login.
    Invalid,
    /// Represents an expired token.
    /// When a token is `Expired`, it needs to be refreshed.
    /// If it cannot be refreshed (e.g. because aux requirements are not satisfied), the user needs to repeat login.
    Expired
}

impl Default for SessionStatus {
    fn default() -> Self {
        SessionStatus::Invalid
    }
}

/// Obtains the status of the session.
pub fn status(token_id: u32) -> Result<SessionStatus> {
    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    let session = dsl::session
        .filter(dsl::token_id.eq(token_id))
        .first::<Session>(&connection);

    match session {
        Ok(session) => {
            if session.expiration >= NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0) {
                Ok(SessionStatus::Expired)
            } else {
                if session.needs_update {
                    Ok(SessionStatus::NeedsUpdate)
                } else {
                    Ok(SessionStatus::Valid)
                }
            }
        },
        Err(diesel::result::Error::NotFound) => Ok(SessionStatus::Invalid),
        Err(_) => Err(Error::InternalServerError().finish())
    }
}

/// Obtains the current session.
pub fn fetch(token_id: u32) -> Result<Session> {
    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    dsl::session
        .filter(dsl::token_id.eq(token_id))
        .first::<Session>(&connection)
        .map_err(|e| {
            if e == diesel::result::Error::NotFound {
                Error::Unauthorized().finish()
            } else {
                Error::InternalServerError().finish()
            }
        })
}

/// Deletes the specified session.
pub fn invalidate(token_id: u32) -> Result<()> {
    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    diesel::delete(
        dsl::session.filter(dsl::token_id.eq(token_id))
    ).execute(&connection)
        .map_err(|_| Error::InternalServerError())?;

    Ok(())
}
/// Modifies the current session status as `NeedsUpdate`.
pub fn update(user_id: u32) -> Result<()> {
    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    diesel::update(
        dsl::session.filter(dsl::user_id.eq(user_id))
    ).set(dsl::needs_update.eq(true))
        .execute(&connection)
        .map_err(|_| Error::InternalServerError())?;

    Ok(())
}

/// Creates a new session.
pub fn create(user_id: u32, exp: i64) -> Result<u32> {
    let session = NewSession {
        user_id,
        expiration: NaiveDateTime::from_timestamp(exp, 0)
    };

    let connection = establish_connection()
        .map_err(|_| Error::InternalServerError())?;

    let token_id: u64 = connection.transaction(|| {
        diesel::insert_into(
            dsl::session
        ).values(&session)
            .execute(&connection)?;

        diesel::select(last_insert_id)
            .first(&connection)
    }).map_err(|_| Error::InternalServerError())?;

    Ok(token_id as u32)
}