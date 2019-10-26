use diesel::backend::Backend;
use diesel::sql_types::VarChar;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::Write;



#[derive(Copy, Clone, Debug, PartialEq, AsExpression, FromSqlRow)]
#[sql_type = "VarChar"]
pub enum Role {
    /// Can do everything.
    Maintainer,
    /// Can do everything except server management.
    Administrator,
    /// Can create new shows.
    Organizer,
    /// Can only observe public shows.
    User,
    /// An user that has not yet been confirmed.
    Pending,
    /// An user that has been banned.
    Banned,
    /// Can do nothing and should repeat log-in.
    Invalid
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match *self {
            Role::Maintainer => "maintainer".to_owned(),
            Role::Administrator => "admin".to_owned(),
            Role::Organizer => "organizer".to_owned(),
            Role::User => "user".to_owned(),
            Role::Pending => "pending".to_owned(),
            Role::Banned => "banned".to_owned(),
            Role::Invalid => "invalid".to_owned()
        }
    }
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match &role.to_lowercase()[..] {
            "maintainer" => Role::Maintainer,
            "admin" => Role::Administrator,
            "organizer" => Role::Organizer,
            "user" => Role::User,
            "pending" => Role::Pending,
            "banned" => Role::Banned,
            _ => Role::Invalid
        }
    }
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match &role.to_lowercase()[..] {
            "maintainer" => Role::Maintainer,
            "admin" => Role::Administrator,
            "organizer" => Role::Organizer,
            "user" => Role::User,
            "pending" => Role::Pending,
            "banned" => Role::Banned,
            _ => Role::Invalid
        }
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

impl<DB: Backend> diesel::serialize::ToSql<VarChar, DB> for Role
    where
        String: diesel::serialize::ToSql<VarChar, DB>
{
    fn to_sql<W: Write>(&self, out: &mut diesel::serialize::Output<W, DB>) -> diesel::serialize::Result {
        self.to_string().to_sql(out)
    }
}

impl<DB: Backend> diesel::deserialize::FromSql<VarChar, DB> for Role
    where
        String: diesel::deserialize::FromSql<VarChar, DB>
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        let role = String::from_sql(bytes)?;
        Ok(Self::from(role))
    }
}