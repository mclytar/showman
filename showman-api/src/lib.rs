//pub mod shows;
macro_rules! allow {
    ($($ident:ident),*) => {
        || HttpResponse::MethodNotAllowed()
            .header("Allow", stringify!($($ident),*))
            .finish()
    }
}

pub(crate) mod self_prelude {
    pub use actix_web::{
        error::{
            self,
            Result
        },
        web::{
            self,
            Data,
            Path,
            Form
        },
        HttpResponse
    };

    pub use showman_data::prelude::*;
}

use self_prelude::*;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg
        // Table `Show`
        .route("/shows", web::get().to(get_shows))
        .route("/shows", web::post().to(post_shows))
        .route("/shows", web::to(allow!(GET, POST)))
        .route("/shows/{id}", web::get().to(get_show_by_id))
        .route("/shows/{id}", web::patch().to(patch_show_by_id))
        .route("/shows/{id}", web::delete().to(delete_show_by_id))
        .route("/shows/{id}", web::to(allow!(GET, PATCH, DELETE)))
        // Table `Scene`
        .route("/scenes/{id}", web::get().to(get_scene_by_id))
        .route("/scenes/{id}", web::patch().to(patch_scene_by_id))
        .route("/scenes/{id}", web::delete().to(delete_scene_by_id))
        .route("/scenes/{id}", web::to(allow!(GET, PATCH, DELETE)))
        .route("/shows/{id}/scenes", web::get().to(get_scenes_of_show))
        .route("/shows/{id}/scenes", web::post().to(post_scenes_in_show))
        .route("/shows/{id}/scenes", web::patch().to(patch_scenes_of_show))
        .route("/shows/{id}/scenes", web::to(allow!(GET, POST, PATCH)))
    ;
    RestChildResource::<CharacterForm, CharacterData, CharacterUpdateForm>::new("characters", "shows").apply(cfg);
    RestChildResource::<PropForm, PropData, PropUpdateForm>::new("props", "shows").apply(cfg);
    RestChildResource::<SoundForm, SoundData, SoundUpdateForm>::new("sounds", "shows").apply(cfg);
    RestChildResource::<TrackForm, TrackData, TrackUpdateForm>::new("tracks", "shows").apply(cfg);
}

// ----------------------------------------------------------------
// `Show`
// ----------------------------------------------------------------
async fn get_shows(dbp: Data<DbPool>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).load_all::<ShowData>().await
}

async fn post_shows(dbp: Data<DbPool>, Form(data): Form<ShowForm>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).create(data, |r| format!("/shows/{}", r)).await
}

async fn get_show_by_id(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).load::<ShowData>(id.into_inner()).await
}

async fn patch_show_by_id(dbp: Data<DbPool>, id: Path<u32>, Form(data): Form<ShowUpdateForm>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).update(data, id.into_inner()).await
}

async fn delete_show_by_id(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).delete::<ShowData>(id.into_inner()).await
}

// ----------------------------------------------------------------
// `Scene`
// ----------------------------------------------------------------
async fn get_scene_by_id(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).load::<SceneData>(id.into_inner()).await
}

async fn patch_scene_by_id(dbp: Data<DbPool>, id: Path<u32>, Form(data): Form<SceneUpdateForm>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).update(data, id.into_inner()).await
}

async fn delete_scene_by_id(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).delete::<SceneData>(id.into_inner()).await
}

async fn get_scenes_of_show(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).load_set::<SceneData>(id.into_inner()).await
}

async fn post_scenes_in_show(dbp: Data<DbPool>, id: Path<u32>, Form(data): Form<SceneForm>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).create_child(data, id.into_inner(), |r| format!("/scenes/{}", r)).await
}

async fn patch_scenes_of_show(dbp: Data<DbPool>, id: Path<u32>, Form(data): Form<SceneUpdateOrderForm>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err(|e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).update(data, id.into_inner()).await
}