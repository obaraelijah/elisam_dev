use std::collections::HashMap;
use std::path::PathBuf;
use std::{ io, fs};
use std::ffi::OsStr;


type Slug = String;

// OrgBlog represents all blog related items.
// See [OrgBlog](crate::blog::OrgBlog) and
// [get_org_blog](crate::blog::get_org_blog).
//
// # Example
//
// let org_blog: OrgBlog = get_org_blog()

#[derive(Serialize, Debug)]
pub struct OrgBlog {
    pub html: HashMap<Slug, OrgModeHtml>,
    // Blog files should be sorted by date (newest is at head)
    pub blog_files: Vec<OrgModeHtml>,
}

// OrgModeHtml represents a particular org-mode blog article.
#[derive(Serialize, Debug, Clone)]
pub struct OrgModeHtml {
    pub title: String,
    pub date: NaiveDate,
    pub pub_date: String,
    pub toc: String,
    pub desc: String,
    pub html: String,
    pub slug: String,
    pub footnotes: Vec<String>,
}

fn get_html_files(base: &str) -> Result<Vec<PathBuf>, io::Error> {
    let base = PathBuf::from(base);
    if !base.is_dir() {
        panic!("BLOG_ROOT is not a directory!")
    }
    let mut html_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        for file in fs::read_dir(path)? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                continue;
            }
            if path.extension().and_then(OsStr::to_str).unwrap_or("") == "html" {
                html_files.push(path);
            }
        }
    }
    Ok(html_files)
}