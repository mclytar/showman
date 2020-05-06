use crate::self_prelude::*;

use crate::schema::sound;

/// Creation form for `Sound` table.
#[derive(CreateChild, Deserialize)]
#[table_name = "sound"]
#[parent_id = "show_id"]
pub struct SoundForm {
    pub name: String,
    pub filename: Option<String>
}

/// Output structure for `Sound` table.
#[derive(Delete, Load, LoadAll, LoadSet, Queryable, Serialize)]
#[table_name = "sound"]
#[parent_id = "show_id"]
pub struct SoundData {
    pub sound_id: u32,
    pub show_id: u32,
    pub name: String,
    pub filename: Option<String>,
}

/// Update form for `Sound` table.
#[derive(Update, AsChangeset, Deserialize)]
#[table_name = "sound"]
pub struct SoundUpdateForm {
    pub name: Option<String>,
    pub filename: Option<String>
}

