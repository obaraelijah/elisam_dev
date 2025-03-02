use rocket::fs::NamedFile;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder, status};
use rocket::Request;
use rocket::Route;
use rocket_dyn_templates::Template;

use crate::context::{get_base_context, get_template};

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

// Allow web crawling
#[get("/robots.txt")]
fn robots_txt() -> &'static str {
    r#"
    # robots.txt
    User-agent: *
    Disallow:
    "#
}

// Custom RSS responder
struct Rss(Template);

impl<'r> Responder<'r, 'static> for Rss {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let mut response = self.0.respond_to(req)?;
        response.set_header(ContentType::new("application", "rss+xml"));
        Ok(response)
    }
}

#[get("/feed")]
fn feed() -> Rss {
    rss()
}

#[get("/rss")]
fn rss() -> Rss {
    let context = get_base_context("/blog");
    Rss(Template::render("blog-rss", context))
}

#[get("/resume_pdf")]
async fn resume_pdf() -> std::io::Result<NamedFile> {
    NamedFile::open(get_template("/resume_pdf")).await
}

#[get("/500")]
fn crash() -> status::Custom<&'static str> {
    status::Custom(Status::InternalServerError, "Server Error")
}

#[get("/blog/<slug>")]
fn blog_article(slug: String) -> Option<Template> {
    let mut context = get_base_context("/blog");
    context.kv.insert("title".to_owned(), "blog".to_owned());
    context.blog.html.get(&slug).map(|curr_blog| {
        context.curr_blog = Some(curr_blog);
        context.kv.insert("curr_slug".to_owned(), slug);
        Template::render("blog/blog_article", context)
    })
}

pub fn get_routes() -> (Vec<Route>, Vec<rocket::Catcher>) {
    (
        routes![
            index,
            resume,
            blog_index,
            linkedin,
            github,
            robots_txt,
            resume_pdf,
            crash,
            blog_article,
            rss,
            feed
        ],
        catchers![not_found, server_err],
    )
}