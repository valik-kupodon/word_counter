pub mod db_models {
    
    use serde::Deserialize;
    use sqlx::FromRow;

    #[derive(Debug, FromRow, Deserialize)]
    pub struct WordCounter {
        id: i32,
        pub word: String,
        pub count: i32,
    }
}
