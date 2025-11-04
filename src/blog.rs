use chrono::NaiveDate;
use log::{
    error,
    info,
};
use select::document::Document;
use select::predicate::{
    Attr,
    Class,
    Name,
};
use serde::Serialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{
    fs,
    io,
};

type Slug = String;

#[derive(Serialize, Debug)]
pub struct OrgBlog {
    pub html: HashMap<Slug, OrgModeHtml>,
    pub blog_files: Vec<OrgModeHtml>,
}

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

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    CannotParseHtml(PathBuf),
    CannotFindTitle(PathBuf),
    CannotFindDate(PathBuf),
    CannotFindToc(PathBuf),
    CannotParseDate(PathBuf),
    CannotFindContents(PathBuf),
    CannotFindFirstParagraph(PathBuf),
    CannotMakeSlug(PathBuf),
}

fn get_html_files(base: &str) -> Result<Vec<PathBuf>, io::Error> {
    let base = PathBuf::from(base);
    if !base.is_dir() {
        panic!("BLOG_ROOT is not a directory!");
    }
    let mut html_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&base)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        for file in fs::read_dir(&path)? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                continue;
            }
            if path.extension().and_then(OsStr::to_str) == Some("html") {
                html_files.push(path);
            }
        }
    }
    Ok(html_files)
}

pub fn get_html_contents(blog_file: &PathBuf) -> Result<OrgModeHtml, ParsingError> {
    let file_contents = fs::read_to_string(blog_file)
        .map_err(|_| ParsingError::CannotParseHtml(blog_file.to_path_buf()))?;
    let document = Document::from(&file_contents[..]);
    info!("Parsing file: {:?}", blog_file);
    info!(
        "File contents preview: {:?}",
        &file_contents[..file_contents.len().min(200)]
    );

    let title = document
        .find(Name("title"))
        .next()
        .or_else(|| document.find(Class("title")).next())
        .or_else(|| document.find(Name("h1")).next())
        .map(|node| node.text())
        .unwrap_or_else(|| {
            blog_file
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        });

    let date_string = document
        .find(Class("timestamp"))
        .next()
        .map(|node| node.text())
        .unwrap_or_else(|| "<1970-01-01 Thu>".to_string());

    let date = NaiveDate::parse_from_str(&date_string, "<%Y-%m-%d %a>").map_err(|e| {
        error!("Could not parse date for {:?}, reason {:?}", date_string, e);
        ParsingError::CannotParseDate(blog_file.to_path_buf())
    })?;

    let pub_date = date.format("%a, %d %b %Y 1:01:00 EST").to_string();

    let toc = document
        .find(Attr("id", "text-table-of-contents"))
        .next()
        .map(|node| node.html())
        .unwrap_or_else(|| "<p>No table of contents available</p>".to_string());

    // Use <body> as fallback if no outline-2, then <html> as last resort
    let outline_node = document
        .find(Class("outline-2"))
        .next()
        .or_else(|| document.find(Name("body")).next())
        .or_else(|| document.find(Name("html")).next())
        .expect("HTML document should have at least an <html> node");

    let desc = outline_node
        .find(Name("p"))
        .next()
        .map(|node| node.text())
        .unwrap_or_else(|| "No description available".to_string());

    let html = outline_node.html();

    let slug_string = blog_file
        .to_str()
        .ok_or(ParsingError::CannotMakeSlug(blog_file.to_path_buf()))?;

    let slug = slug_string
        .split('/')
        .last()
        .ok_or(ParsingError::CannotMakeSlug(blog_file.to_path_buf()))?
        .replace(".html", "");

    let footnotes = document.find(Class("footdef")).map(|x| x.html()).collect();

    info!("Successfully parsed {:?}", blog_file);

    Ok(OrgModeHtml {
        title,
        date,
        pub_date,
        toc,
        desc,
        html,
        slug,
        footnotes,
    })
}

pub fn get_org_mode_files(blog_root: &str) -> Vec<OrgModeHtml> {
    match get_html_files(blog_root) {
        Ok(org) => {
            let html_res: Vec<_> = org.iter().map(|o| get_html_contents(o)).collect();
            let mut html_success: Vec<OrgModeHtml> = Vec::new();
            for html in html_res {
                match html {
                    Ok(h) => html_success.push(h),
                    Err(e) => error!("Failed to parse file {:?}", e),
                }
            }
            html_success.sort_by(|a, b| b.date.cmp(&a.date));
            html_success
        }
        Err(e) => {
            error!("Cannot get org mode files: {:?}", e);
            panic!("Failed to read blog directory: {}", e);
        }
    }
}

pub fn get_org_blog(blog_root: &str) -> OrgBlog {
    let blog_files = get_org_mode_files(blog_root);
    let html: HashMap<Slug, OrgModeHtml> = blog_files
        .clone()
        .into_iter()
        .map(|x| (x.slug.clone(), x))
        .collect();
    OrgBlog { html, blog_files }
}
