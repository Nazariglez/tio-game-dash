use actix_web::{http::Method};
use actix_web::dev::ResourceHandler;
use app::AppState;
use v1::administrators;
use v1::auth;

pub fn get<'a> () -> Vec<(&'a str, fn(&mut ResourceHandler<AppState>))> {
    vec![
        ("/auth/admin/in", |r| {
            r.method(Method::POST).with_async(auth::admin_login);
        }),
        ("/admin", |r| {
            r.method(Method::POST).with_async(administrators::create_admin);
        }),
        ("/admin/{id}", |r| {
            r.method(Method::GET).with_async(administrators::read_admin);
            r.method(Method::DELETE).with_async(administrators::del_admin);
            r.method(Method::PUT).with_async(administrators::update_admin);
        })
    ]
}