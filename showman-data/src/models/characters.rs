use crate::self_prelude::*;

use crate::schema::character;

/// Creation form for `Character` table.
#[derive(CreateChild, Deserialize)]
#[table_name = "character"]
#[parent_id = "show_id"]
pub struct CharacterForm {
    pub name: String,
    pub description: Option<String>
}

/// Output structure for `Character` table.
#[derive(Delete, Load, LoadAll, LoadSet, Queryable, Serialize)]
#[table_name = "character"]
#[parent_id = "show_id"]
pub struct CharacterData {
    pub character_id: u32,
    pub show_id: u32,
    pub name: String,
    pub description: Option<String>,
}

/// Update form for `Character` table.
#[derive(Update, AsChangeset, Deserialize)]
#[table_name = "character"]
pub struct CharacterUpdateForm {
    pub name: Option<String>,
    pub description: Option<String>
}

