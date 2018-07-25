#[macro_use] extern crate log;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
extern crate tio_db;
extern crate tio_api;
extern crate tio_config;

use std::env;
use actix_web::server::HttpServer;
use tio_db::{check_connection, DatabaseParams, run_migrations};
use tio_api::app;

#[cfg(debug_assertions)]
const DEFAULT_CONFIG_FILE: &'static str = "dev.config";

#[cfg(not(debug_assertions))]
const DEFAULT_CONFIG_FILE: &'static str = "prod.config";

fn main() {
    let config_file = env::args().nth(1).unwrap_or(DEFAULT_CONFIG_FILE.to_string()) + ".toml";
    tio_config::init(config_file.clone());
    let config = tio_config::get();
    init_logs(config.log);
    info!("Using config file: {}", config_file);

    check_db(config.database, true);

    let addr = format!("{}:{}", config.server.ip, config.server.port);
    HttpServer::new(|| app::get())
        .bind(addr)
        .expect(&format!("Can not bind to port {}", config.server.port))
        .run();
}

fn check_db(database: tio_config::ConfigDatabase, automigrate: bool) {
    match check_connection(DatabaseParams::new(database.user, database.password, database.host, database.name)) {
        Ok(conn) => {
            if automigrate {
                debug!("Runing database migrations...");
                if let Err(e) = run_migrations(&conn) {
                    error!("Database migration error: {}", e);
                    ::std::process::exit(1);
                }
            }
        },
        Err(e) => {
            error!("Database error: {}", e);
            ::std::process::exit(1);
        }
    }
}

fn init_logs(log: tio_config::ConfigLog) {
    let default_main_level = "info".to_string();
    let default_tarentola_level = "trace".to_string();
    let main_log_level = log.get("main").unwrap_or(&default_main_level);
    let tarentola_log_level = log.get("tarentola").unwrap_or(&default_tarentola_level);

    let mut logs = vec![
        "tio_bin", 
        "tio_db",
        "tio_api",
        "tio_config",

        //add tarentola crates here

    ].into_iter().map(|c| {
        let name = c.to_string();
        (name.clone(), log.get(&name).unwrap_or(tarentola_log_level))
    }).collect::<Vec<_>>();

    for (k, v) in log.iter() {
        if k == "main" || k == "tarentola" { continue; }
        logs.push((k.to_string(), v));
    }

    let custom_logs = logs.into_iter().map(|(k, v)| {
        format!("{}={}", k, v)
    }).collect::<Vec<_>>().join(",");

    let log_crates = format!("{},{}", main_log_level, custom_logs);

    env::set_var("RUST_LOG", &log_crates);
    env_logger::init();

    debug!("Logs enabled: {}", log_crates);
}