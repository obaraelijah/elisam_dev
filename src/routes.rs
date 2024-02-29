use rocket::{Route, Catcher};

#[get("/")]
fn index() -> &'static str {
    "Heath Check!"
}

pub fn get_routes() -> ( Vec<Route>, Vec<Catcher>) {
    (
        routes![
            index,
        ],
        catchers![],
    )
}