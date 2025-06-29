use std::collections::HashMap;

use accept_language::{intersection, parse};
use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::IntoResponse,
};
use tracing::info;

pub async fn get_article(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    Path(article_id): Path<String>,
) -> impl IntoResponse {
    info!("{:?}", headers);

    let lang: &axum::http::HeaderValue = headers.get("accept-language").unwrap();

    let langs = accept_language::parse(lang.to_str().unwrap());

    info!("lang : {:?}", langs);
}


