-- Your SQL goes here
CREATE TABLE news (
  id SERIAL PRIMARY KEY,
  title TEXT,
  link TEXT UNIQUE
)