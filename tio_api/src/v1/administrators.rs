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

pub fn update_admin((id, data, req) : (Path<i32>, Json<EditAdminData>, Request)) -> Response {
    let state_db = req.state().db.clone();
    state_db.send(ReadAdmin {
        id: Some(id.into_inner()),
        email: None
    })
        .from_err()
        .and_then(move |res| {
            match res {
                Ok(admin_to_update) => {
                    //todo check admin level
                    let claims = AuthClaims::from_request(&req)?;
                    let admin_level = claims.admin_level.unwrap();
                    
                    if admin_level > admin_to_update.level {
                        Err(ErrorForbidden("You can't update an admin higher than you"))
                    } else {
                        Ok(UpdateAdmin {
                            id: admin_to_update.id,
                            email: data.email.clone(),
                            password: data.password.clone(),
                            level: data.level.clone()
                        })
                    }
                },
                Err(e) => Err(e)
            }
        }).and_then(move |update_data| {
            state_db.send(update_data)
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(admin) => Ok(Res::OK(admin)),
                        Err(e) => Err(e)
                    }
                })
        }).responder()
}

pub fn del_admin((id, req) : (Path<i32>, Request)) -> Response {
    let state_db = req.state().db.clone();
    //todo can not delete an admin if the admin_level is higher to the request_admin.level
    state_db.send(ReadAdmin {
        id: Some(id.into_inner()),
        email: None
    })
        .from_err()
        .and_then(move |res| {
            match res {
                Ok(admin_to_del) => {
                    //todo check admin level
                    let claims = AuthClaims::from_request(&req)?;
                    let admin_level = claims.admin_level.unwrap();
                    
                    if admin_level > admin_to_del.level {
                        Err(ErrorForbidden("You can't delete an admin higher than you"))
                    } else {
                        Ok(admin_to_del.id)
                    }
                },
                Err(e) => Err(e)
            }
        })
        .and_then(move |admin_id| {
            state_db.send(DeleteAdmin { id : admin_id})
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(_) => Ok(Res::OK("")),
                        Err(e) => Err(e)
                    }
                })
        })
        .responder()
}