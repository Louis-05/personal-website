use std::{collections::HashMap, path::Path};
use anyhow::Result;
use serde::Deserialize;
use crate::{article, lang::Lang};

pub struct Article {
    pub id: String,
    pub categories: Vec<String>,
    pub publication_date: jiff::Zoned,
    pub last_update: jiff::Zoned,
    pub content: HashMap<Lang, ArticleContent>,
}


pub struct ArticleContent {
    pub title: String,
    pub short_content: String,
    pub content: String,
}

#[derive(Debug,Clone,Deserialize)]
pub struct ArticleFile {
    pub id: String,
    pub categories: Vec<String>,
    pub publication_date: jiff::Zoned,
    pub last_update: jiff::Zoned,
}

#[derive(Debug,Clone,Deserialize)]
pub struct ArticleContentFile {
    pub title: String,
    pub content: String,
}


impl Article {
    pub async fn load_from_folder(folder : impl AsRef<Path>) -> Result<Article> {
        let article_path = folder.as_ref().join("article.md");
        let article_content_fr_path = folder.as_ref().join("content-fr.md");
        let article_content_en_path = folder.as_ref().join("content-en.md");
        let media_folder_path = folder.as_ref().join("media");

        
        let article_str = tokio::fs::read_to_string(article_path).await?;
        let article_content_fr_str = tokio::fs::read_to_string(article_content_fr_path).await?;
        let article_content_en_str = tokio::fs::read_to_string(article_content_en_path).await?;

        let article_file: Article = toml::from_str(&article_str)?;
        let article_fr : ArticleContentFile = toml::from_str(&article_content_fr_str)?;
        let article_en : ArticleContentFile = toml::from_str(&article_content_en_str)?;

        let article_cotent_fr = ArticleContent {
            title: article_fr.title,
            short_content: "test".to_string(),
            content: article_fr.content,
        }

    }
}