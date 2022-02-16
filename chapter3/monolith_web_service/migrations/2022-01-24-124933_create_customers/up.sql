-- Your SQL goes here

CREATE TABLE customers (
  id SERIAL PRIMARY KEY,
  customer_name VARCHAR NOT NULL,
  customer_address TEXT NOT NULL
)