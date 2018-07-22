#![allow(dead_code)]
#![allow(non_snake_case)]

use futures::future::{Future, result};
use actix_web::{HttpResponse, Error, AsyncResponder, HttpRequest, Responder};
use serde::ser::*;
use std::fmt::{Debug};
use app::AppState;

pub use actix_web::http::StatusCode;
pub use actix_web::error::*;

pub type Response = Box<Future<Item=HttpResponse, Error=Error>>;
pub type Request = HttpRequest<AppState>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T: Serialize> {
    status: u16,
    message: T
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(status: StatusCode, msg: T) -> ApiResponse<T> {
        ApiResponse {
            status: status.as_u16(),
            message: msg
        }
    }

    pub fn OK(msg: T) -> ApiResponse<T> {
        ApiResponse {
            status: StatusCode::OK.as_u16(),
            message: msg
        }
    }

    pub fn BadRequest(msg: T) -> ApiResponse<T> {
        ApiResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: msg
        }
    }

    pub fn Forbidden(msg: T) -> ApiResponse<T> {
        ApiResponse {
            status: StatusCode::FORBIDDEN.as_u16(),
            message: msg
        }
    }

    pub fn NotFound(msg: T) -> ApiResponse<T> {
        ApiResponse {
            status: StatusCode::NOT_FOUND.as_u16(),
            message: msg
        }
    }

    pub fn InternalServerError(msg: T) -> ApiResponse<T> {
        ApiResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: msg
        }
    }
}

impl<T:Serialize> Responder for ApiResponse<T> {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::build(StatusCode::from_u16(*&self.status).unwrap())
            .content_type("application/json")
            .json(&self)
        )
    }
}

#[inline]
pub fn Res<T: Serialize + Debug> (status: StatusCode, msg: T) -> HttpResponse {
    let res = ApiResponse {
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

#[inline]
pub fn OK<T: Serialize + Debug>(msg: T) -> Response {
    result(Ok(Res(StatusCode::OK, msg))).responder()
}

#[inline]
pub fn BadRequest<T: Serialize + Debug>(msg: T) -> Response {
    result(Ok(Res(StatusCode::BAD_REQUEST, msg))).responder()
}

#[inline]
pub fn Forbidden<T: Serialize + Debug>(msg: T) -> Response {
    result(Ok(Res(StatusCode::FORBIDDEN, msg))).responder()
}

#[inline]
pub fn InternalServerError<T: Serialize + Debug>(msg: T) -> Response {
    result(Ok(Res(StatusCode::INTERNAL_SERVER_ERROR, msg))).responder()
}

#[inline]
pub fn NotFound<T: Serialize + Debug>(msg: T) -> Response {
    result(Ok(Res(StatusCode::NOT_FOUND, msg))).responder()
}