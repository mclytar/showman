use std::collections::HashSet;

use crate::self_prelude::*;

use crate::schema::scene;

/// Creation form for `Scene` table.
#[derive(CreateChild, Deserialize)]
#[table_name = "scene"]
#[parent_id = "show_id"]
#[extern_column(number: i32 <- compute_number)]
pub struct SceneForm {
    pub title: String,
    pub description: Option<String>
}

fn compute_number(dbc: &DbConnection, parent_id: u32) -> diesel::result::QueryResult<i32> {
    use self::scene::dsl::*;
    let pos: Option<i32> = scene.select(diesel::dsl::max(number))
        .filter(show_id.eq(parent_id))
        .first(dbc)?;
    let pos = 1 + pos.unwrap_or(0);
    Ok(pos)
}

/// Output structure for `Scene` table.
#[derive(Delete, Load, LoadAll, LoadSet, Queryable, Serialize)]
#[table_name = "scene"]
#[parent_id = "show_id"]
#[order_by = "number"]
pub struct SceneData {
    pub scene_id: u32,
    pub show_id: u32,
    pub number: i32,
    pub title: String,
    pub description: Option<String>,
}

/// Update form for `Scene` table.
#[derive(Update, AsChangeset, Deserialize)]
#[table_name = "scene"]
pub struct SceneUpdateForm {
    pub title: Option<String>,
    pub description: Option<String>
}

/// Update form for order of `Scene` table
#[derive(Deserialize)]
pub struct SceneUpdateOrderForm {
    pub order: String
}

impl Update for SceneUpdateOrderForm {
    fn update(self, dbc: &DbConnection, id: u32) -> Result<()> {
        use crate::schema::scene::dsl::*;

        let scene_ids: std::result::Result<Vec<u32>, _> = self.order
            .split(',')
            .map(|n| n.parse::<u32>())
            .collect();
        let scene_ids = match scene_ids {
            Ok(ids) => ids,
            Err(_) => return Err(HttpResponse::BadRequest().finish())
        };

        dbc.transaction(|| {
            let curr_ids: Vec<u32> = scene.select(scene_id)
                .filter(show_id.eq(id))
                .load(dbc)?;

            let chk_scene_ids: HashSet<_> = scene_ids.iter().collect();
            let chk_curr_ids: HashSet<_> = curr_ids.iter().collect();

            if chk_curr_ids != chk_scene_ids || curr_ids.len() != scene_ids.len() {
                return Err(DBError::RollbackTransaction);
            }

            let max = scene_ids.len() as i32 + 1;

            diesel::update(scene.filter(show_id.eq(id)))
                .set(number.eq(number + max))
                .execute(dbc)?;

            for (num, sid) in scene_ids.iter().enumerate() {
                diesel::update(scene.filter(scene_id.eq(sid)))
                    .set(number.eq(num as i32 + 1))
                    .execute(dbc)?;
            }

            Ok(())
        }).map_err(|e: DBError| match e {
            DBError::NotFound => HttpResponse::NotFound().finish(),
            DBError::RollbackTransaction => HttpResponse::BadRequest().finish(),
            _ => HttpResponse::InternalServerError().body(format!("{}", e))
        })?;

        Ok(())
    }
}