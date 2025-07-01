use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::IntoResponse,
};
use tracing::info;

use crate::lang::Lang;

pub async fn get_article(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    Path(article_id): Path<String>,
) -> impl IntoResponse {
    info!("{:?}", headers);

    let headerlang: &axum::http::HeaderValue = headers.get("accept-language").unwrap();

    let lang = Lang::get_pref_lang(&headers, &params);

    info!("lang : {:?}", lang);
}
