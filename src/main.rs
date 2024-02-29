#[macro_use]
extern crate rocket;

#[get("/")]
fn health_check() -> &'static str {
    "Heath Check!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
} 