#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;

pub mod models;
pub mod controllers;
pub mod repositories;
pub mod request;
pub mod utils;
pub mod router;

use std::{sync::{Arc, Mutex}};

use repositories::connection::init_pool;
use router::get_routes;
use controllers::user_controller::Cache;

use ttl_cache::TtlCache;

fn main() {
    let token_cache: Cache = Arc::new(Mutex::new(TtlCache::new(500))); 
    rocket::ignite()
        .manage(token_cache)
        .manage(init_pool())
        .mount("/", get_routes())
        .launch();
}