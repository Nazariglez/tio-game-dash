use actix_web::{State, Json, AsyncResponder};
use app::AppState;
use http::*;
use futures::Future;
use tio_db::models::administrators::handlers as admin_handlers;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminLogin {
    pub email: String,
    pub password: String
}

pub fn admin_login((data, state) : (Json<AdminLogin>, State<AppState>)) -> Response {
    state.db.send(admin_handlers::ReadAdmin {
        id: None,
        email: Some(data.email.clone())
    })
        .from_err()
        .and_then(move |res| {
            match res {
                Ok(admin) => {
                    let pass = data.password.clone();
                    if admin_handlers::compare_password(pass, admin.password)? {
                        Ok(Res::OK("LOGED IN"))
                    } else {
                        Err(ErrorNotFound("Invalid email or password."))
                    }
                },
                Err(e) => Err(e)
            }
        }).responder()
}