pub mod text_parsers {

    use std::collections::HashMap;

    pub struct StringParser {
        pub text: String,
    }

    impl StringParser {
        pub fn new(text: String) -> Self {
            Self { text }
        }

        pub fn get_words_count(&self) -> HashMap<String, usize> {
            let words_and_count = self.text.split_whitespace().fold(
                std::collections::HashMap::new(),
                |mut acc, word| {
                    *acc.entry(word.to_owned()).or_insert(0) += 1;
                    acc
                },
            );
            words_and_count
        }
}
}
