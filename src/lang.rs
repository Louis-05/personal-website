use axum::http::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    French,
    English,
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl Lang {
    pub fn from_min_string(string: &str) -> Option<Lang> {
        match string {
            "fr" | "fr-FR" | "fr-CA" => Some(Lang::French),
            "en" | "en-US" | "en-GB" => Some(Lang::English),
            _ => None,
        }
    }


pub fn get_query_pref_lang(params: &HashMap<String,String>) -> Option<Lang> {
    if let Some(lang) = params.get("lang") {
        return Lang::from_min_string(lang);
    } 

    return None;
}

pub fn get_header_pref_lang(headers: &HeaderMap) -> Option<Lang> {
    let Some(header_langs) = headers.get("accept-language") else {
        return None;
    };

    let Some(langs_string) = header_langs.to_str().ok() else {
        return None;
    };

    let langs_strings = accept_language::parse(langs_string);

    let langs: Vec<Lang> = langs_strings
        .iter()
        .filter_map(|s| Lang::from_min_string(s))
        .collect();

    return langs.first().copied();
}

pub fn get_pref_lang(headers: &HeaderMap,params: &HashMap<String,String>) -> Option<Lang> {
    if let Some(lang) = Self::get_query_pref_lang(params) {
        return Some(lang)
    } 
   
    return Self::get_header_pref_lang(headers)
}

}


#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    #[test]
    fn test_lang() {
       assert_eq!(Lang::from_min_string("fr-FR"),Some(Lang::French));
       assert_eq!(Lang::from_min_string("en"),Some(Lang::English));
    }

    #[test]
    fn test_query() {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        assert_eq!(Lang::get_query_pref_lang(&hash_map), None);
        hash_map.insert("lang".to_string(), "fr".to_string());
        assert_eq!(Lang::get_query_pref_lang(&hash_map), Some(Lang::French))
    }

    #[test]
    fn test_header() {
        let mut headers = HeaderMap::new();
        headers.insert("accept-language", HeaderValue::from_str("fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3").unwrap());
        assert_eq!(Lang::get_header_pref_lang(&headers),  Some(Lang::French));
        headers.insert("accept-language", HeaderValue::from_str("en-US,en;q=0.9").unwrap());
        assert_eq!(Lang::get_header_pref_lang(&headers),  Some(Lang::English));
        headers.insert("accept-language", HeaderValue::from_str("de,es-ES;q=0.9").unwrap());
        assert_eq!(Lang::get_header_pref_lang(&headers), None);
    }
}