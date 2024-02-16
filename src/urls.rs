use regex::Regex;

pub fn find_urls(text: &str) -> Vec<String> {
    let url_regex = Regex::new(r"https?://[^\s/$.?#].[^\s\)\(\[\[]\<\>*").unwrap();

    url_regex
        .find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect()
}
