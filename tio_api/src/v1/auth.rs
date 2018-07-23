use actix_web::{State, Json, AsyncResponder};
use app::AppState;
use http::*;
use futures::Future;
use frank_jwt::{encode, decode, Algorithm};
use tio_db::models::administrators::handlers as admin_handlers;
use tio_config;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminLogin {
    pub email: String,
    pub password: String
}

pub fn admin_login((data, state) : (Json<AdminLogin>, State<AppState>)) -> Response {
    let config = tio_config::get();
    let jwt_header = json!({});
    let jwt_payload = json!({
        "admin": true,
        "all_ok": "yes"
    });
    let token = encode(jwt_header, &config.auth.jwt_secret, &jwt_payload, Algorithm::HS256);
    println!("token {:?}", token);
    let de = decode(&token.unwrap(), &config.auth.jwt_secret, Algorithm::HS256);
    println!("de {:?}", de);

    state.db.send(admin_handlers::ReadAdmin {
        id: None,
        email: Some(data.email.clone())
    })
        .from_err()
        .and_then(move |res| {
            match res {
                Ok(admin) => {
                    let pass = data.password.clone();
                    if admin_handlers::compare_password(pass, admin.password)? {
                        Ok(Res::OK("LOGED IN"))
                    } else {
                        Err(ErrorNotFound("Invalid email or password."))
                    }
                },
                Err(e) => Err(e)
            }
        }).responder()
}