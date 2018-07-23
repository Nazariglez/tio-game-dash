use chrono::{NaiveDateTime, Utc};
use time::Duration;
use tio_config;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthClaims {
    pub exp: NaiveDateTime,
    pub id: i32,
    pub admin: Option<bool>,
    pub admin_level: Option<i16>,
    pub dev: Option<bool>,
    pub user: Option<bool>
}

impl AuthClaims {
    pub fn new(id: i32) -> AuthClaims {
        AuthClaims {
            id: id,
            exp: get_expire_token_date(tio_config::get().auth.token_expire as i64),
            admin: None,
            admin_level: None,
            dev: None,
            user: None
        }
    }

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
}

fn get_expire_token_date(expire:i64) -> NaiveDateTime {
    (Utc::now() + Duration::seconds(expire)).naive_utc()
}