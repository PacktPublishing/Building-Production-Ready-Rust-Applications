-- Your SQL goes here

CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  product_name VARCHAR NOT NULL,
  product_id INTEGER NOT NULL,
  customer_id INTEGER NOT NULL,
  amount INTEGER NOT NULL,
  address TEXT NOT NULL,
  fulfilled BOOLEAN NOT NULL DEFAULT 'f'
)