-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  bio TEXT NOT NULL,
  age INTEGER NOT NULL,
  image_url VARCHAR NOT NULL
);
