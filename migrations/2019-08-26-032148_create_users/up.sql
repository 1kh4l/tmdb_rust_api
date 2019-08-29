-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  first_name VARCHAR NOT NULL,
  second_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  birth_date VARCHAR NOT NULL
);
