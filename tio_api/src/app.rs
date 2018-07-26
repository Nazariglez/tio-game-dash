use actix::{Addr, SyncArbiter};
use actix_web::{App, http, pred, AsyncResponder};
use actix_web::middleware::{Logger, cors::Cors};
use tio_db::{create_pool, DatabaseParams, ConnDsl};
use num_cpus;
use tio_config;
use router;
use http::{Request, Response, ErrorNotFound, ErrorMethodNotAllowed};
use middlewares::error_handler::{ApiErrorHandler};
use futures::future::{err};

pub struct AppState {
    pub db: Addr<ConnDsl>
}

pub fn get() -> App<AppState> {
    let cfg = tio_config::get();

    let conn = create_pool(DatabaseParams::new(cfg.database.user, cfg.database.password, cfg.database.host, cfg.database.name)).unwrap();
    let addr = SyncArbiter::start(
        num_cpus::get() * 3, //postgress just allow 8 connections? this value must be 8 instead cpus*n? 
        move || ConnDsl(conn.clone())
    );

    App::with_state(AppState{ db: addr.clone() })
        .middleware(Logger::default())
        .middleware(ApiErrorHandler)
        .configure(|app| {
            let mut cors = Cors::for_app(app);
            cors.allowed_methods(vec![
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::PUT,
                    http::Method::DELETE
                ])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE);

            router::set_routes(&mut cors);
            cors.register()
        }).default_resource(|r| {
            //r.h(http::NormalizePath::default());

            r.method(http::Method::GET).with(default_not_found);
            
            r.route()
                .filter(pred::Not(pred::Get()))
                .with(default_method_not_allowed);
        })
}

fn default_not_found(_: Request) -> Response {
    err(ErrorNotFound("Not Found")).responder()
}

fn default_method_not_allowed(_: Request) -> Response {
    err(ErrorMethodNotAllowed("Method not allowed")).responder()
}