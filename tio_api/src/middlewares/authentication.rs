use http::*;
use actix_web::{HttpRequest};
use actix_web::middleware::{Middleware, Started};
use auth::AuthClaims;
use tio_db::models::administrator_sessions::handlers::*;
use futures::Future;
use app::AppState;

pub struct IsAdmin;

impl Middleware<AppState> for IsAdmin {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        let claims = AuthClaims::from_request(&req)?;
        let is_admin = claims.admin.unwrap_or(false);
        if !is_admin {
            Err(ErrorForbidden("Forbidden"))
        } else {
            Ok(Started::Future(Box::new(
                req.state().db.send(IsValidAdminSession{
                    admin_id: claims.id,
                    update: true
                })
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(is_valid) => {
                            if is_valid {
                                Ok(None)
                            } else {
                                Err(ErrorForbidden("Forbidden"))
                            }
                        },
                        Err(e) => Err(e)
                    }
                })
            )))
        }
    }
}


pub struct IsDev;

impl<S> Middleware<S> for IsDev {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let claims = AuthClaims::from_request(&req)?;
        let is_dev = claims.dev.unwrap_or(false);
        if !is_dev {
            Err(ErrorForbidden("Forbidden"))
        } else {
            Ok(Started::Done) //check db
        }
    }
}


pub struct IsUser;

impl<S> Middleware<S> for IsUser {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let claims = AuthClaims::from_request(&req)?;
        let is_user = claims.user.unwrap_or(false);
        if !is_user {
            Err(ErrorForbidden("Forbidden"))
        } else {
            Ok(Started::Done) //check db
        }
    }
}