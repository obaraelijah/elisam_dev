use rocket_contrib::templates::Template;

use crate::routes::get_routes;

pub fn start_server() -> rocket::Rocket {
    let ( routes, catchers) = get_routes();
    rocket::ignite()
        .mount("/", routes)
        .register(catchers)
        .attach(Template::fairing())
}