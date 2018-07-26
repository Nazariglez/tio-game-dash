use actix_web::{HttpMessage, Error, HttpRequest};
use http::*;
use frank_jwt::{encode, Algorithm, decode, Error as JwtError};
use chrono::{NaiveDateTime, Utc};
use time::Duration;
use tio_config;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthClaims {
    pub exp: NaiveDateTime,
    pub id: i32,
    
    #[serde(skip_serializing_if="Option::is_none")] pub admin: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")] pub admin_level: Option<i16>,
    #[serde(skip_serializing_if="Option::is_none")] pub dev: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")] pub user: Option<bool>
}

impl AuthClaims {
    pub fn admin(id: i32, level: i16) -> AuthClaims {
        AuthClaims {
            id: id,
            exp: get_expire_token_date(tio_config::get().auth.token_expire as i64),
            admin: Some(true),
            admin_level: Some(level),
            dev: None,
            user: None
        }
    }

    pub fn developer(id: i32) -> AuthClaims {
        AuthClaims {
            id: id,
            exp: get_expire_token_date(tio_config::get().auth.token_expire as i64),
            admin: None,
            admin_level: None,
            dev: Some(true),
            user: None
        }
    }

    pub fn user(id: i32) -> AuthClaims {
        AuthClaims {
            id: id,
            exp: get_expire_token_date(tio_config::get().auth.token_expire as i64),
            admin: None,
            admin_level: None,
            dev: None,
            user: Some(true)
        }
    }

    pub fn to_token(&self) -> Result<String, Error> {
        let secret = tio_config::get().auth.jwt_secret;
        encode(json!({}), &secret, &json!(&self), Algorithm::HS256)
            .map_err(|e|{
                error!("Can not generate jwt token for {:?}: {:?}", &self, e);

                ErrorInternalServerError("Error generating auth token")
            })
    }

    pub fn from_token(token:&String) -> Result<AuthClaims, Error> {
        let secret = tio_config::get().auth.jwt_secret;
        let (_, payload) = decode(token, &secret, Algorithm::HS256)
            .map_err(|e|{
                match e {
                    JwtError::SignatureExpired => ErrorUnauthorized("Auth token expired"),
                    _ => ErrorUnauthorized("Invalid auth token")
                }
            })?;

        let claims: AuthClaims = serde_json::from_value(payload)
            .map_err(ErrorInternalServerError)?;

        if claims.exp < Utc::now().naive_utc() {
            Err(ErrorUnauthorized("Auth token expired"))
        } else {
            Ok(claims)
        }
    }

    pub fn from_request<S>(req: &HttpRequest<S>) -> Result<AuthClaims, Error> {
        let headers = req.headers();
        if let Some(auth) = headers.get("authorization") {
            let header = auth.to_str().map_err(ErrorForbidden)?;
                if !header.starts_with("Bearer: ") {
                Err(ErrorUnauthorized("Invalid auth token"))
            } else {
                let token = header.replace("Bearer: ", "");
                let claims = AuthClaims::from_token(&token)?;

                Ok(claims)
            }
        } else {
            Err(ErrorUnauthorized("Invalid authorization header"))
        }
    }
}

fn get_expire_token_date(expire:i64) -> NaiveDateTime {
    (Utc::now() + Duration::seconds(expire)).naive_utc()
}