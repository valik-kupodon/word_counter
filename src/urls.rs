use std::collections::HashMap;

use crate::database::{create_word_count, truncate_word_count, get_word_count};
use crate::schemas::WordCount;
use crate::schemas::{ParseTextRequest, WordCountRequest};
use actix_web::rt::task;
use actix_web::{error, get, post, web, Result};
use log::{error, info};
use sqlx::PgPool;
use crate::services::text_parser::text_parsers::{StringParser, get_words_count_map_from_file};
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

#[post("/parse_text_from_file")]
pub async fn parse_text_from_file(file: web::Bytes, pool: web::Data<PgPool>) -> Result<Json<HashMap<String, usize>>> {
    let chunk_size = file.len() / 8;
    let futures = (0..8).map(|i| {
        let start = i * chunk_size;
        let end = if i == 7 {
            file.len()
        } else {
            (i + 1) * chunk_size
        };
        let chunk = file.slice(start..end);
        task::spawn_blocking(move || {
            get_words_count_map_from_file(&chunk)
        })
    }).collect::<Vec<_>>();
    let mut word_counter_maps = Vec::new();
    for future in futures {
        match future.await {
            Ok(result) => {
                info!("Result: {:?}", result);
                word_counter_maps.push(result);
            }
            Err(err) => {
                error!("{:?}", err);
            }
        }
    }
    let word_counter_map = word_counter_maps.into_iter().flat_map(|map| map.into_iter()).fold(HashMap::new(), |mut acc, (word, count)| {
        *acc.entry(word).or_insert(0) += count;
        acc
    });
    let word_counts = word_counter_map.iter().map(|(word, count)| {
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

    Ok(Json(word_counter_map))
}
