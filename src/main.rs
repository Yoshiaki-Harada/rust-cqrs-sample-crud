#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod app;
pub mod domain;
mod schema;
fn main() {
    app::start();
}
