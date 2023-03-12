#[macro_use]
extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "Welcome to the landing page!"
}

#[get("/hello")]
fn hello_world() -> &'static str {
    "hello world"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong!"
}

#[get("/predict")]
fn predict() -> &'static str {
    "Temporary Output"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])
        .mount("/", routes![hello_world])
        .mount("/", routes![ping])
        .mount("/", routes![predict])
}
