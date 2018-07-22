use actix_web::{HttpMessage, AsyncResponder};
use futures::Future;
use http::*;
use tio_db::models::administrators::handlers::*;

pub fn create_admin(req: Request) -> Response {
    let state_db = req.state().db.clone();
    req.json()
        .from_err()
        .and_then(move |data: CreateAdmin |{
            state_db.send(data)
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(admin) => Ok(Res(StatusCode::OK, admin)),
                        Err(e) => Err(ErrorBadRequest(e))
                    }
                })
        })
        .responder()
}