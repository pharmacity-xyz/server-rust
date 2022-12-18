-- Add migration script here
CREATE TABLE orders (
    order_id uuid PRIMARY KEY,
    user_id TEXT REFERENCES users (user_id),
    total_price NUMERIC NOT NULL,
    ship_address TEXT NOT NULL,
    order_date TIMESTAMPTZ NOT NULL,
    shipped_date TIMESTAMPTZ NOT NULL
);
