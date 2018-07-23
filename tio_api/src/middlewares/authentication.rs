use http::*;
use actix_web::{HttpRequest, HttpMessage};
use actix_web::middleware::{Middleware, Started};
use actix_web::http::header::HeaderValue;
use frank_jwt::{encode, decode, Algorithm, Error as JwtError};
use tio_config;

pub struct IsAdmin;

impl<S> Middleware<S> for IsAdmin {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        let headers = req.headers();
        if let Some(auth) = headers.get("authorization") {
            let a = auth.to_str().map_err(ErrorForbidden)?;
            if !a.starts_with("Bearer: ") {
                return Err(ErrorForbidden("Invalid auth token"));
            }

            let secret = tio_config::get().auth.jwt_secret;
            let token = a.replace("Bearer: ", "");
            let (_, payload) = decode(&token, &secret, Algorithm::HS256)
                .map_err(|e|{
                    match e {
                        JwtError::SignatureExpired => ErrorForbidden("Auth token expired"),
                        _ => ErrorForbidden("Invalid auth token")
                    }
                })?;

            println!("{:?}", payload);

            Ok(Started::Done)
        } else {
            Err(ErrorForbidden("Forbidden"))
        }
    }
}