#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;

pub mod app;
pub mod domain;
fn main() {
    app::start();
}
