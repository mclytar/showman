#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate showman_data_derive;

pub mod crud;
pub mod models;
pub mod result;
pub mod schema;

pub(crate) mod self_prelude {
    pub use actix_web::HttpResponse;
    pub use chrono::NaiveDateTime;
    pub use diesel::prelude::*;
    pub use diesel::result::Error as DBError;
    pub use diesel::result::DatabaseErrorKind as DBErrorKind;

    pub use crate::DbConnection;
    pub use crate::last_insert_id;
    pub use crate::crud::*;
    pub use crate::result::Result;
}

pub mod prelude {
    pub use crate::DbPool;
    pub use crate::crud::*;
    pub use crate::models::shows::*;
    pub use crate::models::characters::*;
    pub use crate::models::props::*;
    pub use crate::models::scenes::*;
    pub use crate::models::sounds::*;
    pub use crate::models::tracks::*;
}

use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

no_arg_sql_function!(last_insert_id, diesel::sql_types::Unsigned<diesel::sql_types::Integer>);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
