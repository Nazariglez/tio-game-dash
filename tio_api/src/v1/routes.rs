use actix_web::{http::Method};
use actix_web::dev::Resource;
use app::AppState;
use v1::administrators;
use v1::auth;
use middlewares::authentication::IsAdmin;

pub fn get<'a> () -> Vec<(&'a str, fn(&mut Resource<AppState>))> {
    vec![
        ("/auth/admins/in", |r| {
            r.post().with_async(auth::admin_login);
        }),

        ("/admins", |r| {
            r.middleware(IsAdmin);

            r.method(Method::GET).with_async(administrators::get_administrators);
            r.method(Method::POST).with_async(administrators::create_admin);
        }),

        ("/admins/{id}", |r| {
            r.middleware(IsAdmin);

            r.method(Method::GET).with_async(administrators::read_admin);
            r.method(Method::DELETE).with_async(administrators::del_admin);
            r.method(Method::PUT).with_async(administrators::update_admin);
        })
    ]
}