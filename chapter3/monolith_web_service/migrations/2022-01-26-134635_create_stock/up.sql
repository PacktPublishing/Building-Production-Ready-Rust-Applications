-- Your SQL goes here

CREATE TABLE stocks (
  id SERIAL PRIMARY KEY,
  product_name VARCHAR NOT NULL,
  product_id INTEGER NOT NULL,
  amount INTEGER NOT NULL
)