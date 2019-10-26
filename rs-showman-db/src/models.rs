pub mod role;

use diesel::Queryable;

use role::Role;

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct Authentication {
    pub auth_id: u32,
    pub user_id: u32,
    pub method: String,
    pub user_data: String,
    pub token: String
}

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct User {
    pub user_id: u32,
    pub name: String,
    pub surname: String,
    pub role: Role
}