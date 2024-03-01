use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::Serialize;


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
pub struct SiteContext {
    /// base is the static key-value context of the website.
    /// All of the information in base comes from
    /// [STATIC_SITE_CONTEXT_KV](crate::context::STATIC_SITE_CONTEXT_KV)
    pub base: &'static SiteContextKv,
    /// kv is the dynamic key-value context of the website.
    pub kv: SiteContextKv,
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
    static ref STATIC_SITE_CONTEXT_KV: SiteContextKv = {
       site_context! {
        "nav_site_href" =>  "/"
       }  
    };
}

//get base context
pub fn get_base_context(nav_href_uri: &str) -> SiteContext {
    SiteContext {
        base: &STATIC_SITE_CONTEXT_KV,
        // TODO: Not waste memory like this
        kv: {
            let mut tmp = SiteContextKv::new();
            tmp.insert("nav_site_href".to_owned(), nav_href_uri.to_owned());
            tmp
        },

    }
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
        "/" => "index"
    };
}

pub fn get_template(uri: &str) -> &str {
    TEMPLATE_MAP.get(uri).unwrap()
}