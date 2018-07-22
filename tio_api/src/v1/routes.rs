use actix_web::{http::Method};
use actix_web::dev::ResourceHandler;
use app::AppState;
use v1::users;
use v1::administrators;

pub fn get<'a> () -> Vec<(&'a str, fn(&mut ResourceHandler<AppState>))> {
    vec![
        ("/user", |r| {
            r.method(Method::POST).with_async(users::create_user);
        }),
        ("/user/{id}", |r| {
            r.method(Method::GET).with_async(users::get_user);
            r.method(Method::DELETE).with_async(users::delete_user);
            r.method(Method::PUT).with_async(users::update_user);
        }),
        ("/admin", |r| {
            r.method(Method::POST).with_async(administrators::create_admin);
        }),
    ]
}