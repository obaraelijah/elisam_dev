use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

use crate::routes::get_routes;

pub fn start_server() -> rocket::Rocket<rocket::Build> {
    let (routes, catchers) = get_routes();
    rocket::build()
        .mount("/", routes)
        .mount("/static", FileServer::from("static"))
        .register("/", catchers)
        .attach(Template::fairing())
}