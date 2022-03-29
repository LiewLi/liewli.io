use anyhow::{Context, Ok, Result};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Options, Parser};
use std::io::Write;

use handlebars::Handlebars;
use std::collections::HashMap;

fn template_html(content: &str) -> Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("md", include_str!("./template/md.hbs"))?;
    let mut data: HashMap<&str, String> = HashMap::new();
    data.insert("content", content.into());
    let html = handlebars.render("md", &data)?;
    Ok(html)
}

fn md_to_html(path: &Path) -> Result<String> {
    let opts = Options::empty();
    let input = fs::read_to_string(path).unwrap();
    let mut p = Parser::new_ext(&input, opts);
    let mut str_buffer = std::io::BufWriter::new(Vec::with_capacity(1024 * 1024));
    html::write_html(str_buffer.by_ref(), &mut p)?;
    let str = String::from_utf8(str_buffer.into_inner()?)?;
    template_html(&str)
}

fn conv_to_html(src: &DirEntry, dst: &Path) -> Result<()> {
    let name = src.file_name().to_str().unwrap().to_owned();
    if fs::read_dir(dst).is_ok() {
        fs::remove_dir_all(dst).context(format!("remove {:?} failed", dst))?;
    }

    fs::create_dir_all(dst).context(format!("create {:?} failed", dst))?;

    let md_contents: Vec<DirEntry> = fs::read_dir(src.path())
        .unwrap()
        .filter_map(|p| p.ok())
        .filter(|p| p.file_name().to_str().unwrap() == format!("{name}.md"))
        .collect();

    let resources: Vec<DirEntry> = fs::read_dir(src.path())
        .unwrap()
        .filter_map(|p| p.ok())
        .filter(|p| p.file_name().to_str().unwrap().ends_with(".md") == false)
        .collect();

    for p in &md_contents {
        if let Some(html) = md_to_html(&p.path()).ok() {
            let mut dst_html = PathBuf::from(&dst);
            let path = p.path();
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            dst_html.push(format!("{file_name}.html"));
            fs::write(&dst_html, html)?;
        }
    }

    for res in &resources {
        let path = res.path();
        let mut dst_res = PathBuf::from(&dst);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        dst_res.push(file_name);
        fs::copy(path, &dst_res)?;
    }
    Ok(())
}

fn gen_index(mds: Vec<String>, dst: &Path) -> Result<()> {
    let mut mds = mds;
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("index", include_str!("./template/index.hbs"))?;
    let mut data: HashMap<&str, String> = HashMap::new();
    mds.sort();
    let list = mds.iter().map(|md| {
        format!("<li><a href=\"./html/{md}/{md}.html\">{md}</a></li>")
    }).collect();
    data.insert("entries", list);
    let html = handlebars.render("index", &data)?;
    fs::write(dst, html)?;
    Ok(())
}

pub fn gen(url: &str) -> Result<()> {
    let root = PathBuf::from(url);
    let mut md_dir = root.clone();
    md_dir.push("md");

    let mut html_dir = root.clone();
    html_dir.push("html");

    if !md_dir.is_dir() || !html_dir.is_dir() {
        eprintln!("md not found");
        return Ok(());
    }

    let mds = fs::read_dir(md_dir)?;

    let md_entries: Vec<DirEntry> = mds.filter_map(|p| p.ok()).collect();

    let md_html_names:Vec<String> = (&md_entries)
        .into_iter()
        .map(|f| f.file_name())
        .filter_map(|f| f.into_string().ok())
        .collect();

    let mut index_dst = root.clone();
    index_dst.push("index.html");

    gen_index(md_html_names, &index_dst)?;

    for src in md_entries {
        let mut dst = html_dir.clone();
        dst.push(src.file_name());
        if let Err(e) = conv_to_html(&src, &dst) {
            eprintln!("convert {:?} failed, error: {:?}", src.file_name(), e);
        }
    }

    Ok(())
}
