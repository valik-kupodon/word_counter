use std::collections::HashMap;

use crate::database::{create_word_count, truncate_word_count, get_word_count};
use crate::schemas::WordCount;
use crate::error::JsonDecodeError;
use crate::schemas::{ParseTextRequest, WordCountRequest};
use crate::services::get_text_type;
use actix_web::{error, get, post, web, Result};
use log::{error, info};
use sqlx::PgPool;
use crate::services::text_parser::text_parsers::StringParser;
use actix_web::web::Json;

#[get("/")]
pub async fn index() -> Result<String> {
    let body = "Hello, welcome to the text parser services! Please use the following endpoints:\n\nGET /get_count_of_word/{{word}}\nPOST /parse_text\n\n".to_string();
    Ok(body)
}

#[post("/get_count_of_word")]
pub async fn get_count_of_word(body: web::Json<WordCountRequest>, pool: web::Data<PgPool>) -> Result<Json<HashMap<String, usize>>> {
    let words = body.words.iter().map(|word| (word.to_lowercase().to_owned())).collect::<Vec<String>>();
    let word_counts = get_word_count(pool.clone(), &words).await;
    match word_counts {
        Ok(word_counts) => {
            Ok(Json(word_counts.iter().map(|word_count| (word_count.word.to_owned(), word_count.count as usize)).collect::<HashMap<String, usize>>()))
        }
        Err(err) => {
            error!("{:?}", err);
            Err(error::ErrorInternalServerError("Cannot get word count"))
        }
    }
}

#[post("/parse_text")]
pub async fn parse_text(body: web::Json<ParseTextRequest>, pool: web::Data<PgPool>) -> Result<Json<HashMap<String, usize>>> {
    info!("POST /parse_text {:?}", body);
    println!("{:?}", body);
    let text_type = get_text_type(body.text_type.as_str());
    if text_type.is_none() {
        let result = Err(JsonDecodeError {
            name: "Validation error on field: text_type\nerror: For text_type 'text', 'string', 'URL' available only"
        });
        error!("{:?}", result);
        return result.map_err(|err| error::ErrorBadRequest(err.name));
    }
    info!("Text type: {:?}", text_type);
    match text_type.unwrap().as_str() {
        "string" => {
            let text = body.text.as_str();
            let parser = StringParser::new(text.to_owned());
            let result = parser.get_words_count();
            let word_counts = result.iter().map(|(word, count)| {
                WordCount {
                    word: word.to_owned(),
                    count: *count as i32,
                }
            }).collect::<Vec<WordCount>>();
            info!("Start truncating table");
            let _ = truncate_word_count(pool.clone()).await.or_else(|_| Err(error::ErrorInternalServerError("Error truncating table")));
            info!("Done truncating table");
            info!("Start creating word count");
            let _ = create_word_count(pool.clone(), word_counts).await.or_else(|_| Err(error::ErrorInternalServerError("Error creating word count")));

            Ok(Json(result))
        }
        // "text" => {
        //     let file_path = body.text.as_str();
        //     info!("File path: {:?}", file_path);
        //     let result = Ok("File path".to_owned());
        //     result
        // }
        // "URL" => {
        //     let url = body.text.as_str();
        //     info!("URL: {:?}", url);
        //     let result = Ok("URL".to_owned());
        //     result
        // }
        _ => {
            let result = Err(JsonDecodeError {
                name: "Validation error on field: text_type\nerror: For text_type 'text', 'string', 'URL' available only"
            });
            result.map_err(|err| error::ErrorBadRequest(err.name))
        }
    }
}
