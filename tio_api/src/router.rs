use actix_web::dev::Resource;
use actix_web::middleware::cors::CorsBuilder;
use app::AppState;
use v1;

pub fn set_routes(app: &mut CorsBuilder<AppState>) {
    add(app, "api/v1", v1::routes::get());
}

pub fn add<'a> (app: &mut CorsBuilder<AppState>, prefix: &str, routes: Vec<(&'a str, fn(&mut Resource<AppState>))>) {
    for (route, handler) in routes {
        app.resource(&format!("{}{}", prefix, route), handler);
    }
}