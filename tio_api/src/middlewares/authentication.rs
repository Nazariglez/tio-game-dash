use http::*;
use actix_web::{HttpRequest, HttpMessage};
use actix_web::middleware::{Middleware, Started};
use auth::AuthClaims;

pub struct IsAdmin;

impl<S> Middleware<S> for IsAdmin {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        let claims = AuthClaims::from_request(&req)?;
        let is_admin = claims.admin.unwrap_or(false);
        if !is_admin {
            Err(ErrorForbidden("Forbidden"))
        } else {
            Ok(Started::Done)
        }
    }
}


pub struct IsDev;

impl<S> Middleware<S> for IsDev {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        let claims = AuthClaims::from_request(&req)?;
        let is_dev = claims.dev.unwrap_or(false);
        if !is_dev {
            Err(ErrorForbidden("Forbidden"))
        } else {
            Ok(Started::Done)
        }
    }
}


pub struct IsUser;

impl<S> Middleware<S> for IsUser {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        let claims = AuthClaims::from_request(&req)?;
        let is_user = claims.user.unwrap_or(false);
        if !is_user {
            Err(ErrorForbidden("Forbidden"))
        } else {
            Ok(Started::Done)
        }
    }
}