use rocket::{Route, Catcher};
use rocket_contrib::templates::Template;

use crate::context::{get_base_context, get_template};


#[get("/")]
fn index() -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("title".to_owned(), "home".into());
    Template::render(get_template("/"), context)
}

pub fn get_routes() -> ( Vec<Route>, Vec<Catcher>) {
    (
        routes![
            index,
        ],
        catchers![],
    )
}