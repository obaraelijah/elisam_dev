use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::Serialize;

use crate::blog::{get_org_blog, OrgBlog, OrgModeHtml};

/// BLOG_ROOT is the relative path to blog
pub static BLOG_ROOT: &str = "blog/";

/// SiteContextKv represents all key-value variables used in
/// this project.
///
/// # Example
///
/// let mut foo = SiteContextKv::new()
/// foo.insert("key".to_owned(), "value".to_owned())
type SiteContextKv = HashMap<String, String>;

/// TemplateMap adds some indirection between
/// routes and the actual templates used in the project.
// See [get_template](crate::context::get_template).
///
/// # Example
///
/// let template: &'static str = get_template("/blog")
/// assert_eq!(template, "blog/blog_root")
type TemplateMap = HashMap<&'static str, &'static str>;


#[derive(Serialize, Debug)]
pub struct SiteContext<'a> {
    /// base is the static key-value context of the website.
    /// All of the information in base comes from
    /// [STATIC_SITE_CONTEXT_KV](crate::context::STATIC_SITE_CONTEXT_KV)
    pub base: &'static SiteContextKv,
    /// kv is the dynamic key-value context of the website.
    pub kv: SiteContextKv,
    /// blog is all blog related items, see [OrgBlog](crate::context::OrgBlog)
    pub blog: &'static OrgBlog,
    /// curr_blog is the current blog article, if applicable.
    pub curr_blog: Option<&'a OrgModeHtml>,
}

macro_rules! site_context(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = SiteContextKv::new();
            $(
                m.insert($key.to_owned(), $value.to_owned());
            )+
                m
        }
    };
);

lazy_static! {
    static ref STATIC_BLOG_ENTRIES: OrgBlog = get_org_blog(BLOG_ROOT);
}

lazy_static! {
    static ref STATIC_SITE_CONTEXT_KV: SiteContextKv = {
        site_context! {
            "domain_name" =>  "realelijahobara.tech",
            "nav_site_href" =>  "/",
            "root_uri" =>  "/",
            "blog_uri" =>  "/blog",
            "resume_uri" =>  "/resume",
            "linkedin_uri" =>  "/linkedin",
            "github_uri" =>  "/github",
            "resume_uri" =>  "/resume",
            "resume_pdf_uri" =>  "/resume_pdf",
            "rss_uri" =>  "/rss",
            "crash_uri" =>  "/500",
            "web_sep" =>  "--",
            "admin_email" =>  "elijahobara357@gmail.com",
            "full_name" =>  "Elijah Samson",
            "internet_handle" =>  "elijahsam",
            "my_email" =>  "elijahobara357@gmail.com",
            "github_url" => "https://github.com/obaraelijah",
            "github_repo_url" => "https://github.com/obaraelijah/elisam_dev",
            "linkedin_url" => "https://www.linkedin.com/in/elijah-samson-16619912a/"
        }
    };
}

//get base context
pub fn get_base_context(nav_href_uri: &str) -> SiteContext<'_> {
    SiteContext {
        base: &STATIC_SITE_CONTEXT_KV,
        // TODO: Not waste memory like this.
        kv: {
            let mut tmp = SiteContextKv::new();
            tmp.insert("nav_site_href".to_owned(), nav_href_uri.to_owned());
            tmp
        },
        blog: &STATIC_BLOG_ENTRIES,
        curr_blog: None,
    }
}

pub fn init_context() {
    println!("{}", get_base_context("/").blog.blog_files.len());
}

macro_rules! template_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = TemplateMap::new();
            $(
                m.insert($key, $value);
            )+
                m
        }
    };
);

lazy_static! {
    pub static ref TEMPLATE_MAP: TemplateMap = template_map! {
        "/" => "index",
        "404" => "404",
        "500" => "500",
        "/blog" => "blog/blog_root",
        "/linkedin" => "linkedin",
        "/github" => "github",
        "/resume_pdf" => "resume/elijah_resume.pdf",
        "/resume" => "resume"
    };
}