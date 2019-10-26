pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use showman_core::error::{Error, Result};

pub fn establish_connection() -> Result<MysqlConnection> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::InternalServerError())?;
    let connection = MysqlConnection::establish(&database_url)
        .map_err(|_| Error::InternalServerError())?;
    Ok(connection)
}