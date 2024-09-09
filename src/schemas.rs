use serde_derive::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Deserialize, Debug)]
pub struct ParseTextRequest {
    pub text_type: String,
    pub text: String,
}

#[derive(Deserialize, Debug, Clone, FromRow)]
pub struct WordCount {
    pub word: String,
    pub count: i32,
}

#[derive(Deserialize, Debug)]
pub struct WordCountRequest {
    pub words: Vec<String>,
}
