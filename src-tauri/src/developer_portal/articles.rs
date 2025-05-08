use tauri::AppHandle;
use std::path::PathBuf;
use regex::Regex;
use deunicode::deunicode;

const ARTICLES_DIR: &str = "content/articles";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Article {
    title: String,
    content: String,
    date: String,
    draft: bool,
    file_name: String,
}

#[tauri::command]
pub async fn get_articles(repo_path: String) -> Result<Vec<Article>, String> {
    let path = PathBuf::from(repo_path).join(ARTICLES_DIR);
    let mut articles = Vec::new();

    for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.join("_index.md").exists() {
            let content = std::fs::read_to_string(path.join("_index.md")).map_err(|e| e.to_string())?;
            let article = parse_article(&content, path.file_name().unwrap().to_str().unwrap())?;
            articles.push(article);
        }
    }
    Ok(articles)
}

#[tauri::command]
pub async fn save_article(article: Article, repo_path: String) -> Result<(), String> {
    let normalized_file_name = create_file_name(&article.title);
    let path = PathBuf::from(repo_path).join(ARTICLES_DIR).join(&normalized_file_name).join("_index.md");
    let content = format!(
        r#"+++
title = '{}'
date = '{}'
draft = {}
+++
{}
"#,
        article.title, article.date, article.draft, article.content
    );
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_article(file_name: String, repo_path: String) -> Result<(), String> {
    let path = PathBuf::from(repo_path).join(ARTICLES_DIR).join(&file_name);
    std::fs::remove_dir_all(path).map_err(|e| e.to_string())?;
    Ok(())
}

fn parse_article(content: &str, file_name: &str) -> Result<Article, String> {
    let re = Regex::new(r"(?s)\+\+\+(?P<meta>.+?)\+\+\+(?P<body>.*)").map_err(|e| e.to_string())?;
    let caps = re.captures(content).ok_or("Invalid article format")?;
    let meta = caps.name("meta").ok_or("Invalid article format")?.as_str();
    let body = caps.name("body").map_or("", |m| m.as_str()).trim().to_string();
    let mut article = Article {
        title: String::new(),
        content: body,
        date: String::new(),
        draft: false,
        file_name: file_name.to_string(),
    };

    for line in meta.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches(|c| c == '"' || c == '\'');
            match key {
                "title" => article.title = value.to_string(),
                "date" => article.date = value.to_string(),
                "draft" => article.draft = value.parse::<bool>().map_err(|e| e.to_string())?,
                _ => {}
            }
        }
    }
    Ok(article)
}

fn create_file_name(name: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9-]").unwrap();
    let normalized_name = deunicode(name).to_lowercase().replace(" ", "-");
    re.replace_all(&normalized_name, "").to_string()
}
