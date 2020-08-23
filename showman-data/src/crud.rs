use serde::{Serialize};

use crate::self_prelude::*;

pub trait Create {
    fn create(self, dbc: &DbConnection) -> Result<u32>;
}

pub trait CreateChild {
    fn create_child(self, dbc: &DbConnection, parent_id: u32) -> Result<u32>;
    fn parent_resource_name() -> &'static str;
}

pub trait Load: Sized {
    fn load(dbc: &DbConnection, id: u32) -> Result<Self>;
}

pub trait LoadAll: Sized {
    fn load_all(dbc: &DbConnection) -> Result<Vec<Self>>;
}

pub trait LoadSet: Sized {
    fn load_set(dbc: &DbConnection, parent_id: u32) -> Result<Vec<Self>>;
}

pub trait Update {
    fn update(self, dbc: &DbConnection, id: u32) -> Result<()>;
}

pub trait Delete {
    fn delete(dbc: &DbConnection, id: u32) -> Result<()>;
}

pub struct HttpInterface<'a>(pub &'a DbConnection);

impl<'a> HttpInterface<'a> {
    pub async fn create<C, F>(&self, data: C, location: F) -> Result<HttpResponse>
        where
        C: Create,
        F: Fn(u32) -> String {
        data.create(self.0)
            .map(|r| HttpResponse::Created()
                //.header("Access-Control-Allow-Origin", "*")
                .header("Location", location(r))
                .finish())
    }

    pub async fn create_child<C, F>(&self, data: C, parent_id: u32, location: F) -> Result<HttpResponse>
        where
        C: CreateChild,
        F: Fn(u32) -> String {
        data.create_child(self.0, parent_id)
            .map(|r| HttpResponse::Created()
                //.header("Access-Control-Allow-Origin", "*")
                .header("Location", location(r))
                .finish())
    }

    pub async fn load<L>(&self, id: u32) -> Result<HttpResponse>
        where L: Load + Serialize {
        L::load(self.0, id)
            .map(|r| HttpResponse::Ok()
                //.header("Access-Control-Allow-Origin", "*")
                .json(r))
    }

    pub async fn load_all<L>(&self) -> Result<HttpResponse>
        where L: LoadAll + Serialize {
        L::load_all(self.0)
            .map(|r| HttpResponse::Ok()
                //.header("Access-Control-Allow-Origin", "*")
                .json(r))
    }

    pub async fn load_set<L>(&self, parent_id: u32) -> Result<HttpResponse>
        where L: LoadSet + Serialize {
        L::load_set(self.0, parent_id)
            .map(|r| HttpResponse::Ok()
                //.header("Access-Control-Allow-Origin", "*")
                .json(r))
    }

    pub async fn update<U>(&self, data: U, id: u32) -> Result<HttpResponse>
        where U: Update {
        data.update(self.0, id)
            .map(|_| HttpResponse::NoContent()
                //.header("Access-Control-Allow-Origin", "*")
                .finish())
    }

    pub async fn delete<D>(&self, id: u32) -> Result<HttpResponse>
        where D: Delete {
        D::delete(self.0, id)
            .map(|_| HttpResponse::NoContent()
                //.header("Access-Control-Allow-Origin", "*")
                .finish())
    }
}

use std::marker::PhantomData;

use actix_web::{
    web::{
        self,
        ServiceConfig,
        Data,
        Path,
        Form
    }
};
use serde::Deserialize;

use crate::DbPool;

pub struct RestChildResource<C, R, U> {
    name: String,
    parent: String,
    _t: PhantomData<(C, R, U)>
}

impl<C, R, U> RestChildResource<C, R, U>
where
    C: CreateChild + 'static,
    R: Delete + Load + LoadSet + LoadAll + Serialize + 'static,
    U: Update + 'static,
    for<'de> C: Deserialize<'de>,
    for<'de> U: Deserialize<'de> {
    pub fn new<S1, S2>(name: S1, parent: S2) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str> {
        let name = name.as_ref().to_owned();
        let parent = parent.as_ref().to_owned();
        let _t = PhantomData;
        RestChildResource { name, parent, _t }
    }
    pub fn apply(self, cfg: &mut ServiceConfig) {
        let res_all = format!("/{}", self.name);
        let res_one = format!("/{}/{{id}}", self.parent);
        let res_set = format!("/{}/{{id}}/{}", self.parent, self.name);

        // ---- HTTP /collection/{id}
        // GET
        cfg.route(&res_one, web::get().to(get_one::<R>));
        // PATCH
        cfg.route(&res_one, web::patch().to(patch_one::<U>));
        // DELETE
        cfg.route(&res_one, web::delete().to(delete_one::<R>));
        // ---- HTTP /parent/{id}/collection
        // GET
        cfg.route(&res_set, web::get().to(get_children::<R>));
        // POST
        cfg.route(&res_set, web::post().to(post_child::<C>));
        // ---- HTTP /collection
        // GET
        cfg.route(&res_all, web::get().to(get_all::<R>));
        // ---- `Method Not Allowed` for missing methods
        cfg.route(&res_one, web::to(|| HttpResponse::MethodNotAllowed()
            .header("Allow", "GET, PATCH, DELETE")
            .finish()));
        cfg.route(&res_set, web::to(|| HttpResponse::MethodNotAllowed()
            .header("Allow", "GET, POST")
            .finish()));
        cfg.route(&res_all, web::to(|| HttpResponse::MethodNotAllowed()
            .header("Allow", "GET")
            .finish()));
    }
}

pub async fn get_one<R: Load + Serialize>(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err::<Error, _>( |e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).load::<R>(id.into_inner()).await
}

pub async fn patch_one<U: Update>(dbp: Data<DbPool>, id: Path<u32>, Form(data): Form<U>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err::<Error, _>( |e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).update(data, id.into_inner()).await
}

pub async fn delete_one<D: Delete>(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err::<Error, _>( |e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).delete::<D>(id.into_inner()).await
}

pub async fn get_children<R: LoadSet + Serialize>(dbp: Data<DbPool>, id: Path<u32>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err::<Error, _>( |e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).load_set::<R>(id.into_inner()).await
}

pub async fn post_child<C: CreateChild>(dbp: Data<DbPool>, id: Path<u32>, Form(data): Form<C>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err::<Error, _>( |e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).create_child(data, id.into_inner(), |r| format!("/{}/{}", C::parent_resource_name(), r)).await
}

pub async fn get_all<R: LoadAll + Serialize>(dbp: Data<DbPool>) -> Result<HttpResponse> {
    let dbc = dbp.get()
        .map_err::<Error, _>( |e| error::ErrorServiceUnavailable(e.to_string()))?;
    HttpInterface(&dbc).load_all::<R>().await
}