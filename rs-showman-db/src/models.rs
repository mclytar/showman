pub mod role;

use chrono::NaiveDateTime;
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

#[derive(Clone, Debug, Queryable)]
pub struct Show {
    pub show_id: u32,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub creation: NaiveDateTime
}

#[derive(Clone, Debug, Serialize)]
pub struct ShowData {
    pub id: u32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub notes: String,
    pub access: Option<bool>
}

impl ShowData {
    pub fn restrict(&mut self) {
        self.title = None;
        self.subtitle = None;
        self.description = None;
        self.access = Some(false);
    }

    pub fn grant(&mut self) {
        self.access = Some(true);
    }
}

impl From<Show> for ShowData {
    fn from(show: Show) -> Self {
        let Show {show_id, title, subtitle, description, notes, ..} = show;

        let id = show_id;
        let notes = notes.unwrap_or(String::new());
        let title = Some(title);

        ShowData {id, title, subtitle, description, notes, access: None}
    }
}

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct User {
    pub user_id: u32,
    pub name: String,
    pub surname: String,
    pub role: Role
}