#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate serde_derive;
extern crate bcrypt;
extern crate chrono;
extern crate actix;
extern crate actix_web;
extern crate uuid;

mod schema;
pub mod models;

use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::Connection;
use actix::{Actor, SyncContext};

embed_migrations!("./migrations");

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct ConnDsl(pub PgPool);
impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}

#[derive(Clone, Debug)]
pub struct DatabaseParams {
    pub user: String,
    pub password: String,
    pub host: String,
    pub name: String
}

impl DatabaseParams {
    pub fn new(user:String, pass:String, host:String, name:String) -> DatabaseParams {
        DatabaseParams {
            user: user,
            password: pass,
            host: host,
            name: name
        }
    }
}

pub fn run_migrations(conn: &PgConnection) -> Result<(), String> {
    embedded_migrations::run_with_output(conn, &mut std::io::stdout())
        .map_err(|e| e.to_string())
}

/*
pub fn redo_migrations(conn: &PgConnection) -> Result<(), String>  {
    embedded_migrations::revert(conn)
        .map_err(|e| e.to_string())
}*/

pub fn create_pool(params: DatabaseParams) -> Result<PgPool, String> {
    Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(get_url(params)))
        .map_err(|err| err.to_string())
}

pub fn check_connection(params: DatabaseParams) -> Result<PgConnection, String> {
    PgConnection::establish(&get_url(params))
        //.map(|v| v)
        .map_err(|err| err.to_string())
}

fn get_url(p: DatabaseParams) -> String {
    format!(
        "postgres://{user}:{password}@{host}/{database}",
        user = p.user,
        password = p.password,
        host = p.host,
        database = p.name
    )
}