use actix_web::{AsyncResponder, Json, State, Path};
use futures::Future;
use futures::future::{err};
use http::*;
use tio_db::models::administrators::handlers::*;
use app::AppState;
use auth::AuthClaims;

pub fn create_admin((data, req) : (Json<CreateAdmin>, Request)) -> Response {
    let mut data = data;
    let r_claims = AuthClaims::from_request(&req);
    if r_claims.is_err() {
        return err(r_claims.unwrap_err()).responder();
    }

    //admins can't create more admin with smaller or equal level
    let admin_level = r_claims.unwrap().admin_level.unwrap();
    if let Some(lvl) = data.level {
        if lvl <= admin_level {
            data.level = Some(admin_level+1);
        }
    } else {
        data.level = Some(admin_level+1);
    }

    req.state().db.send(data.into_inner())
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
    //todo can not update an admin if the admin_level is higher to the request_admin.level
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
    //todo can not delete an admin if the admin_level is higher to the request_admin.level
    state.db.send(DeleteAdmin { id : id.into_inner()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(admin) => Ok(Res::OK(admin)),
                Err(e) => Err(e)
            }
        }).responder()
}