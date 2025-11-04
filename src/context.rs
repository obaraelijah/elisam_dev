use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashMap;

use crate::blog::{get_org_blog, OrgBlog, OrgModeHtml};

/// BLOG_ROOT is the relative path to blog
pub static BLOG_ROOT: &str = "blog/";

/// SiteContextKv represents all key-value variables used in this project.
type SiteContextKv = HashMap<String, String>;

/// TemplateMap adds indirection between routes and templates.
type TemplateMap = HashMap<&'static str, &'static str>;

#[derive(Serialize, Debug)]
pub struct SiteContext<'a> {
    pub base: &'static SiteContextKv,
    pub kv: SiteContextKv,
    pub blog: &'static OrgBlog,
    pub curr_blog: Option<&'a OrgModeHtml>,
}

macro_rules! site_context {
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = SiteContextKv::new();
            $(
                m.insert($key.to_owned(), $value.to_owned());
            )+
            m
        }
    };
}

lazy_static! {
    static ref STATIC_BLOG_ENTRIES: OrgBlog = get_org_blog(BLOG_ROOT);
}

lazy_static! {
    static ref STATIC_SITE_CONTEXT_KV: SiteContextKv = {
        site_context! {
            "domain_name" => "realelijahobara.tech",
            "nav_site_href" => "/",
            "root_uri" => "/",
            "blog_uri" => "/blog",
            "resume_uri" => "/resume",
            "linkedin_uri" => "/linkedin",
            "github_uri" => "/github",
            "resume_pdf_uri" => "/resume_pdf",
            "rss_uri" => "/rss",
            "crash_uri" => "/500",
            "web_sep" => "--",
            "admin_email" => "elijahobara357@gmail.com",
            "full_name" => "Elijah Samson",
            "internet_handle" => "elijah samson",
            "my_email" => "elijahobara357@gmail.com",
            "github_url" => "https://github.com/obaraelijah",
            "github_repo_url" => "https://github.com/obaraelijah/elisam_dev",
            "linkedin_url" => "https://www.linkedin.com/in/elijah-samson-16619912a/"
        }
    };
}

pub fn get_base_context(nav_href_uri: &str) -> SiteContext<'_> {
    let mut kv = SiteContextKv::with_capacity(1); // Pre-allocate for one entry
    kv.insert("nav_site_href".to_owned(), nav_href_uri.to_owned());
    SiteContext {
        base: &STATIC_SITE_CONTEXT_KV,
        kv,
        blog: &STATIC_BLOG_ENTRIES,
        curr_blog: None,
    }
}



