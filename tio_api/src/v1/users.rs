use http::*;
//use app::State;
use actix_web::{Either, Error, error, HttpMessage, Path, AsyncResponder, FromRequest, Json, Query, Form};
use tio_utils::is_valid_email;
use futures::Future;
use futures::future::{ok};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCreateInfo {
    pub is_anonymous: bool,
    pub anonymous_id: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub repeat_password: Option<String>
}

pub fn create_user(req: Request) -> Response {
    /*let state_db = req.state().db.clone();

    req.json().from_err()
        .and_then(move |d: UserCreateInfo| {
            let fn_done = |res|{
                match res {
                    Ok(u) => Ok(Res(StatusCode::OK, u)),
                    Err(e) => Ok(Res(StatusCode::BAD_REQUEST, e))
                }
            };

            //register new user 
            if d.anonymous_id.is_none() {
                let mut create_user = users::CreateUser {
                    is_anonymous: d.is_anonymous,
                    email: None,
                    password: None,
                    with_anonymous_id: None,
                };

                //use email and password if exists, otherwise use an anomyous_id
                if !d.is_anonymous {
                    if let Err(e) = can_register_by_email(d.clone()) {
                        return Either::A(ok(Res(StatusCode::BAD_REQUEST, e)));
                    }

                    create_user.email = d.email;
                    create_user.password = d.password;
                }


                return Either::B(state_db.send(create_user)
                    .from_err()
                    .and_then(fn_done)
                );
            }

            //update an anonymous_id with an email and password
            let anon_id = d.anonymous_id.clone().unwrap();
            if let Err(_) = Uuid::parse_str(anon_id.as_str()) {
                return Either::A(ok(Res(StatusCode::BAD_REQUEST, "Invalid anonymous_id")));
            }
            
            if let Err(e) = can_register_by_email(d.clone()) {
                return Either::A(ok(Res(StatusCode::BAD_REQUEST, e)));
            }

            //todo register email with an existent anonymous_id
            //todo update
            let create_user = users::CreateUser {
                is_anonymous: false,
                with_anonymous_id: d.anonymous_id,
                email: d.email,
                password: d.password,
            };

            Either::B(state_db.send(create_user)
                .from_err()
                .and_then(fn_done)
            )

        })
        .responder()*/
        OK("")
}

fn can_register_by_email(data: UserCreateInfo) -> Result<(), String> {
    if data.email.is_none() {
        return Err("Email is required.".to_string());
    }

    if data.password.is_none() {
        return Err("Password is required.".to_string());
    }

    if data.repeat_password.is_none() {
        return Err("Passwords do not match.".to_string());
    }

    let pass = data.password.unwrap();
    let rpass = data.repeat_password.unwrap();
    if pass != rpass {
        return Err("Passwords do not match.".to_string());
    }

    let email = data.email.unwrap();
    if !is_valid_email(email.as_str()) {
        return Err("Invalid email".to_string());
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct UserPublicInfo {
    email: Option<String>,
    id: i32,
    created_at: String,
    updated_at: String
}

pub fn get_user(req: Request) -> Response {
    /*let params = Path::<i32>::extract(&req);
    if let Err(_) = params {
        return BadRequest("Invalid user id");
    }

    req.state().db.send(users::UserInfo{
        id: Some(params.unwrap().into_inner()),
        email: None,
        anonymous_id: None
    }).and_then(|res| {
            match res {
                Ok(user) => {
                    let info = UserPublicInfo {
                        id: user.id,
                        email: user.email,
                        created_at: user.created_at.to_string(),
                        updated_at: user.updated_at.to_string()
                    };

                    Ok(Res(StatusCode::OK, info))
                },
                Err(e) => Ok(Res(StatusCode::BAD_REQUEST, e))
            }
        })
        .or_else(|e| {
            Ok(Res(StatusCode::BAD_REQUEST, e.to_string()))
        })
        .responder()*/
        OK("")
}

pub fn delete_user(id: Path<u64>) -> Response {
    //todo
    OK(format!("{}", id.into_inner()))
}

pub fn update_user(id: Path<u64>) -> Response {
    //todo
    OK(format!("{}", id.into_inner()))
}