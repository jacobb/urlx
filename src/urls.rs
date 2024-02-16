use regex::Regex;
use std::collections::HashSet;

pub fn find_urls(text: &str, dedupe: &bool) -> Vec<String> {
    let url_regex = Regex::new(r#"https?://[^\s/$.?#].[^\s\)\(\[\[><"]*"#).unwrap();

    let urls = url_regex
        .find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect();

    if !dedupe {
        return urls;
    }
    let unique: HashSet<_> = urls.into_iter().collect();
    unique.into_iter().collect()
}
