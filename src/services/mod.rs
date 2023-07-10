pub fn get_text_type(text_type: &str) -> Option<String> {
    match text_type {
        "string" => Some("string".to_owned()),
        "text" => Some("file_path".to_owned()),
        "URL" => Some("url".to_owned()),
        _ => None,
    }
}
