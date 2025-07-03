use crate::lang::Lang;
use anyhow::Result;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
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

        let article = Self::convert_files_to_article(article_file, contents_files);

        Ok(article)
    }

    fn convert_files_to_article(
        article_file: ArticleFile,
        contents_files: HashMap<Lang, String>,
    ) -> Article {
        let mut contents: HashMap<Lang, ArticleContent> = HashMap::new();

        for (lang, md_content) in contents_files {
            let article_content = Self::convert_file_to_content(&md_content);
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

    fn convert_file_to_content(contents_files: &str) -> ArticleContent {
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);

        let parser = Parser::new_ext(&contents_files, options);

        let mut current: Option<MyEvent> = None;
        let mut title = String::new();

        let iter = parser.map(|e| match e {
            Event::Start(tag) => {
                match &tag {
                    Tag::Heading {
                        level,
                        id,
                        classes,
                        attrs,
                    } => {
                        if *level == HeadingLevel::H1 {
                            current = Some(MyEvent::Title);
                        }
                    }
                    _ => (),
                }
                Event::Start(tag)
            }
            Event::End(tag_end) => {
                match &tag_end {
                    pulldown_cmark::TagEnd::Heading(heading_level) => {
                        if *heading_level == HeadingLevel::H1 {
                            current = None;
                        }
                    }
                    _ => (),
                }
                Event::End(tag_end)
            }
            Event::Text(cow_str) => {
                match current {
                    Some(MyEvent::Title) => title.push_str(&cow_str),
                    _ => (),
                }
                Event::Text(cow_str)
            }
            e => e,
        });

        //TODO add support for medias

        let mut html_buf = String::new();
        pulldown_cmark::html::push_html(&mut html_buf, iter);

        //TODO add title find
        let article_content = ArticleContent {
            title: "TODO".to_string(),
            short_content: "TODO".to_string(),
            content: html_buf,
        };

        article_content
    }
}
pub enum MyEvent {
    Title,
    Other,
}
