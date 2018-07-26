use diesel::result::Error as DieselError;
use actix_web::{Error, error};

pub mod administrators;
pub mod administrator_sessions;

pub fn negotiate_error(err: DieselError) -> Error {
    match err {
        DieselError::NotFound => error::ErrorNotFound("Not found"),
        _ => error::ErrorInternalServerError(err)
    }
}