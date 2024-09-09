-- Add migration script here
  CREATE TABLE
    IF NOT EXISTS word_counter (
      id SERIAL PRIMARY KEY,
      word VARCHAR(255) NOT NULL,
      count INT NOT NULL
  );
