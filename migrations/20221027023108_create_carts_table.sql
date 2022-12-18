-- Add migration script here
CREATE TABLE cart_items (
	user_id TEXT NOT NULL REFERENCES users (user_id),
    product_id TEXT NOT NULL REFERENCES products (product_id),
    quantity INT NOT NULL
);
