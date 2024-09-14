pub mod text_parsers {

    use std::collections::HashMap;
    use log::{info, error};
    use regex::Regex;
    use lazy_static::lazy_static;

    pub struct StringParser {
        pub text: String,
    }

    impl StringParser {
        pub fn new(text: String) -> Self {
            Self { text }
        }

        pub fn get_words_count(&self) -> HashMap<String, usize> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(?i)\b[\w`]+(?:['-][\w`]+)*\b").unwrap();
            }
            RE.captures_iter(&self.text).fold(HashMap::new(), |mut word_count, caps| {
                if let Some(word) = caps.get(0) {
                    *word_count.entry(word.as_str().to_lowercase()).or_insert(0) += 1;
                }
                word_count
            })
    }

    }

    pub fn get_words_count_map_from_file(file: &[u8]) -> HashMap<String, usize> {
        let mut text = String::new();
        let chunck_size = 512;
        let mut chunck_start = 0;
        let mut buffer = Vec::new();
        while chunck_start < file.len() {
            let mut chunck_end = (chunck_start + chunck_size).min(file.len());
            while chunck_end < file.len() && file[chunck_end - 1] != b' ' {
                chunck_end += 1;
            }
            buffer.extend_from_slice(&file[chunck_start..chunck_end]);
            if let Ok(chunck_string) = String::from_utf8(buffer.clone()) {
                text.push_str(&chunck_string);
                buffer.clear();
            } else {
                error!("Could not parse chunck");
                buffer.clear();
                continue;
            }
            chunck_start = chunck_end;
        }
        info!("POST /parse_text_from_file {:?}", text);
        let parser = StringParser::new(text.to_owned());
        parser.get_words_count()
    }
}
