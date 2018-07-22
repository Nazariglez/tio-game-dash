#![allow(dead_code)]
#![allow(non_snake_case)]

use futures::future::{Future};
use actix_web::{HttpResponse, Error, HttpRequest};
use serde::ser::*;
use std::fmt::{Debug};
use app::AppState;
use std::marker::{Sync, Send};
use std::fmt::Display;

pub use actix_web::http::StatusCode;
pub use actix_web::error::*;

pub type Response = Box<Future<Item=HttpResponse, Error=Error>>;
pub type Request = HttpRequest<AppState>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Res<T: Serialize> {
    status: u16,
    message: T
}

impl<T: Serialize + Debug> Res<T> {
    #[inline]
    pub fn create(status: StatusCode, msg: T) -> HttpResponse {
        create_response(status, msg)
    }

    #[inline]
    pub fn error(status: StatusCode, err: T) -> Error 
    where
        T: Send + Sync + Debug + Display + 'static {

        InternalError::new(err, status).into()
    }

    #[inline]
    pub fn OK(msg: T) -> HttpResponse {
        create_response(StatusCode::OK, msg)
    }

    #[inline]
    pub fn BadRequest(err: T) -> Error 
    where
        T: Send + Sync + Debug + Display + 'static {

        ErrorBadRequest(err)
    }

    #[inline]
    pub fn Forbidden(err: T) -> Error 
    where
        T: Send + Sync + Debug + Display + 'static {
            
        ErrorForbidden(err)
    }

    #[inline]
    pub fn NotFound(err: T) -> Error 
    where
        T: Send + Sync + Debug + Display + 'static {
            
        ErrorNotFound(err)
    }

    #[inline]
    pub fn InternalServerError(err: T) -> Error 
    where
        T: Send + Sync + Debug + Display + 'static {
            
        ErrorInternalServerError(err)
    }

    #[inline]
    pub fn MethodNotAllowed(err: T) -> Error 
    where
        T: Send + Sync + Debug + Display + 'static {
            
        ErrorMethodNotAllowed(err)
    }
}

#[inline]
fn create_response<T: Serialize + Debug> (status: StatusCode, msg: T) -> HttpResponse {
    let res = Res {
        status: status.as_u16(),
        message: msg
    };

    if status == StatusCode::INTERNAL_SERVER_ERROR {
        error!("{:?}", res);
    } else {
        debug!("{:?}", res);
    }

    HttpResponse::build(status)
        .content_type("application/json")
        .json(res)
}