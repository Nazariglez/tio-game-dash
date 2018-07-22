use actix_web::{HttpResponse, HttpRequest, Result};
use actix_web::middleware::{Middleware, Response};
use http::{Res, StatusCode};

pub struct ApiErrorHandler;

impl<S> Middleware<S> for ApiErrorHandler {
    fn response(&self, _: &mut HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
        if resp.error().is_some() {
            let status = resp.status();
            let err = resp.error().unwrap().to_string();

            match status {
                StatusCode::INTERNAL_SERVER_ERROR => error!("Status: {} - {:?}", status.as_u16(), err),
                _ => warn!("Status: {} - {:?}", status.as_u16(), err)
            }

            Ok(Response::Done(Res(status, err)))
        } else {
            Ok(Response::Done(resp))
        }
    }
}