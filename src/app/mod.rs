use rocket;
pub fn start() {
    rocket::ignite()
    .mount("/hello", routes![hello])
    .launch();
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}