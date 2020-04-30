use rocket;
use crate::domain::Resolver;

pub mod todos;

pub fn start() {
    rocket::ignite()
    .manage(Resolver::default())
    .mount("/hello", routes![hello])
    .mount("/todo", routes![todos::get])
    .launch();
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}