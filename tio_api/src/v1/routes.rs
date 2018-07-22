use actix_web::{http::Method};
use actix_web::dev::ResourceHandler;
use app::AppState;
use v1::administrators;

pub fn get<'a> () -> Vec<(&'a str, fn(&mut ResourceHandler<AppState>))> {
    vec![
        ("/admin", |r| {
            r.method(Method::POST).with_async(administrators::create_admin);
        }),
    ]
}