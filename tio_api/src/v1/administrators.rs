use actix_web::{AsyncResponder, Json, State, Path};
use futures::Future;
use http::*;
use tio_db::models::administrators::handlers::*;
use app::AppState;

pub fn create_admin((data, state) : (Json<CreateAdmin>, State<AppState>)) -> Response {
    state.db.send(data.into_inner())
        .from_err()
        .and_then(|res| {
            match res {
                Ok(admin) => Ok(Res::OK(admin)),
                Err(e) => Err(e)
            }
        }).responder()
}

pub fn read_admin((id, state) : (Path<i32>, State<AppState>)) -> Response {
    state.db.send(ReadAdmin {
        id: Some(id.into_inner()),
        email: None
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(admin) => Ok(Res::OK(admin)),
                Err(e) => Err(e)
            }
        }).responder()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditAdminData {
    pub email: Option<String>,
    pub password: Option<String>,
    pub level: Option<i16>
}

pub fn update_admin((id, data, state) : (Path<i32>, Json<EditAdminData>, State<AppState>)) -> Response {
    state.db.send(UpdateAdmin {
        id: id.into_inner(),
        email: data.email.clone(),
        password: data.password.clone(),
        level: data.level.clone()
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(admin) => Ok(Res::OK(admin)),
                Err(e) => Err(e)
            }
        }).responder()
}

pub fn del_admin((id, state) : (Path<i32>, State<AppState>)) -> Response {
    state.db.send(DeleteAdmin { id : id.into_inner()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(admin) => Ok(Res::OK(admin)),
                Err(e) => Err(e)
            }
        }).responder()
}