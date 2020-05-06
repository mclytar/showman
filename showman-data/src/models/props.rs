use crate::self_prelude::*;

use crate::schema::prop;

/// Creation form for `Prop` table.
#[derive(CreateChild, Deserialize)]
#[table_name = "prop"]
#[parent_id = "show_id"]
pub struct PropForm {
    pub name: String,
    pub description: Option<String>
}

/// Output structure for `Prop` table.
#[derive(Delete, Load, LoadAll, LoadSet, Queryable, Serialize)]
#[table_name = "prop"]
#[parent_id = "show_id"]
pub struct PropData {
    pub prop_id: u32,
    pub show_id: u32,
    pub name: String,
    pub description: Option<String>,
}

/// Update form for `Prop` table.
#[derive(Update, AsChangeset, Deserialize)]
#[table_name = "prop"]
pub struct PropUpdateForm {
    pub name: Option<String>,
    pub description: Option<String>
}

