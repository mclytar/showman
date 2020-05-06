use crate::self_prelude::*;

use crate::schema::track;

/// Creation form for `Track` table.
#[derive(CreateChild, Deserialize)]
#[table_name = "track"]
#[parent_id = "show_id"]
pub struct TrackForm {
    pub title: String,
    pub live: bool,
    pub filename: Option<String>
}

/// Output structure for `Track` table.
#[derive(Delete, Load, LoadAll, LoadSet, Queryable, Serialize)]
#[table_name = "track"]
#[parent_id = "show_id"]
pub struct TrackData {
    pub track_id: u32,
    pub show_id: u32,
    pub title: String,
    pub live: bool,
    pub filename: Option<String>,
}

/// Update form for `Track` table.
#[derive(Update, AsChangeset, Deserialize)]
#[table_name = "track"]
pub struct TrackUpdateForm {
    pub title: Option<String>,
    pub live: bool,
    pub filename: Option<String>
}

