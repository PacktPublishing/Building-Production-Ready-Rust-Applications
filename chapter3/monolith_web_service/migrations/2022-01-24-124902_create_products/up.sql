-- Your SQL goes here

CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  product_name VARCHAR NOT NULL,
  product_type VARCHAR NOT NULL,
  amount INTEGER NOT NULL
)