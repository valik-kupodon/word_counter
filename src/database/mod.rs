pub mod models;
use models::db_models::WordCounter;
use sqlx::PgPool;
use actix_web::{web, Result};
use crate::schemas::WordCount;

pub async fn create_word_count(pool: web::Data<PgPool>, word_count: Vec<WordCount>) -> Result<(), sqlx::Error> {
    let tranasation = pool.begin().await?;
    let chunk_size = 1000;
    let chunks = word_count.chunks(chunk_size);
    for chunk in chunks{
        for word_count in chunk {
            sqlx::query!(
                r#"
                INSERT INTO word_counter (word, count)
                VALUES ($1, $2)
                "#,
                word_count.word,
                word_count.count,
            )
            .execute(pool.get_ref())
            .await?;
        }
    }
    tranasation.commit().await?;
    Ok(())
}

pub async fn truncate_word_count(pool: web::Data<PgPool>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        TRUNCATE TABLE word_count
        "#
    )
    .execute(pool.get_ref())
    .await?;
    Ok(())
}

pub async fn get_word_count(pool: web::Data<PgPool>, words: &Vec<String>) -> Result<Vec<WordCounter>, sqlx::Error> {
    let sql_query = "SELECT * FROM word_counter WHERE word = ANY($1)".to_string();
    let word_count = sqlx::query_as::<_, WordCounter>(&sql_query)
        .bind(words)
        .fetch_all(pool.get_ref())
        .await?;

    Ok(word_count)
}
