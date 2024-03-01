use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::NamedFile;
use rocket::http::Status;

use crate::context::{get_base_context, get_template};
use rocket::{http, Request};
use rocket::{Route, Catcher};


#[get("/")]
fn index() -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("title".to_owned(), "home".into());
    Template::render(get_template("/"), context)
}

#[get("/resume")]
fn resume() -> Template {
    let mut context = get_base_context("/resume");
    context.kv.insert("title".to_owned(), "resume".into());
    Template::render(get_template("/resume"), context)
}

#[get("/blog")]
fn blog_index() -> Template {
    let mut context = get_base_context("/blog");
    context.kv.insert("title".to_owned(), "blog".into());
    Template::render(get_template("/blog"), context)
}

#[get("/linkedin")]
fn linkedin() -> Template {
    let mut context = get_base_context("/linkedin");
    context.kv.insert("title".to_owned(), "linkedin".into());
    Template::render(get_template("/linkedin"), context)
}

#[get("/github")]
fn github() -> Template {
    let mut context = get_base_context("/github");
    context.kv.insert("title".to_owned(), "github".into());
    Template::render(get_template("/github"), context)
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("uri".to_owned(), req.uri().to_string());
    context.kv.insert("title".to_owned(), "404".to_owned());
    Template::render(get_template("404"), context)
}

#[catch(500)]
fn server_err(req: &Request<'_>) -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("uri".to_owned(), req.uri().to_string());
    context.kv.insert("title".to_owned(), "500".to_owned());
    Template::render(get_template("500"), context)
}

// allow web crawling
#[get("/robots.txt")]
fn robots_txt() -> &'static str {
    r#"
    # robots.txt
    User-agent: *
    Disallow:
    "#
}

#[derive(Responder)]
struct Rss {
    inner: Template,
    header: http::ContentType,
}

impl Rss {
    fn new(inner: Template) -> Self {
        Self {
            inner,
            header: http::ContentType::new("application", "rss+xml"),
        }
    }
}

#[get("/feed")]
fn feed() -> Rss {
    rss()
}

#[get("/rss")]
fn rss() -> Rss {
    let context = get_base_context("/blog");
    Rss::new(Template::render("blog-rss", context))
}

pub fn get_routes() -> ( Vec<Route>, Vec<Catcher>) {
    (
        routes![
            index,
            resume,
            blog_index,
            linkedin,
            github,
            robots_txt,
            rss,
            feed
        ],
        catchers![not_found, server_err],
    )
}
