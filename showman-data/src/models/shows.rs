use crate::self_prelude::*;

use crate::schema::show;

/// Creation form for `Show` table.
#[derive(Create, Insertable, Deserialize)]
#[table_name = "show"]
pub struct ShowForm {
    pub title: String,
    pub description: Option<String>,
    pub creation: Option<NaiveDateTime>
}

/// Output structure for `Show` table.
#[derive(Delete, Load, LoadAll, Queryable, Serialize)]
#[table_name = "show"]
pub struct ShowData {
    pub show_id: u32,
    pub title: String,
    pub description: Option<String>,
    pub creation: NaiveDateTime
}

/// Update form for `Show` table.
#[derive(Update, AsChangeset, Deserialize)]
#[table_name = "show"]
pub struct ShowUpdateForm {
    pub title: Option<String>,
    pub description: Option<String>
}