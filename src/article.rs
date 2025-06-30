use crate::lang::Lang;
use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

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

#[derive(Debug, Clone, Deserialize)]
pub struct ArticleFile {
    pub id: String,
    pub categories: Vec<String>,
    pub publication_date: jiff::Zoned,
    pub last_update: jiff::Zoned,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArticleContentFile {
    pub title: String,
    pub content: String,
}

impl Article {
    pub async fn load_from_folder(folder: impl AsRef<Path>) -> Result<Article> {
        let article_path = folder.as_ref().join("article.md");
        let article_content_fr_path = folder.as_ref().join("content-fr.md");
        let article_content_en_path = folder.as_ref().join("content-en.md");

        //TODO MEDIA let media_folder_path = folder.as_ref().join("media");

        let article_str = tokio::fs::read_to_string(article_path).await?;
        let article_file: ArticleFile = toml::from_str(&article_str)?;

        let mut contents_files: HashMap<Lang, String> = HashMap::new();
        contents_files.insert(
            Lang::English,
            tokio::fs::read_to_string(article_content_en_path).await?,
        );
        contents_files.insert(
            Lang::French,
            tokio::fs::read_to_string(article_content_fr_path).await?,
        );

        let article = Self::convert_file(article_file, contents_files);

        Ok(article)
    }

    fn convert_file(article_file: ArticleFile, contents_files: HashMap<Lang, String>) -> Article {
        let mut contents: HashMap<Lang, ArticleContent> = HashMap::new();

        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);

        for (lang, md_content) in contents_files {
            let parser = pulldown_cmark::Parser::new_ext(&md_content, options);

            //TODO add support for medias

            let mut html_buf = String::new();
            pulldown_cmark::html::push_html(&mut html_buf, parser);

            //TODO add title find
            let article_content = ArticleContent {
                title: "TODO".to_string(),
                short_content: "TODO".to_string(),
                content: html_buf,
            };

            contents.insert(lang, article_content);
        }

        let article = Article {
            id: article_file.id,
            categories: article_file.categories,
            publication_date: article_file.publication_date,
            last_update: article_file.last_update,
            content: contents,
        };

        return article;
    }
}
