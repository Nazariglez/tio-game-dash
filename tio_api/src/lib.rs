//#![feature(proc_macro, generators)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
//#[macro_use] extern crate futures_await;
extern crate serde;
extern crate actix;
extern crate actix_web;
extern crate num_cpus;
extern crate futures;
extern crate uuid;
extern crate tio_db;
extern crate tio_utils;
extern crate tio_config;

pub mod v1;
pub mod middlewares;

pub mod app;
pub mod router;

mod http;