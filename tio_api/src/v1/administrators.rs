use actix_web::{AsyncResponder, Json, State};
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
                Err(e) => Err(Res::BadRequest(e))
            }
        }).responder()
}