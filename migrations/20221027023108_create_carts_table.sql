-- Add migration script here
CREATE TABLE cart_items (
	user_id uuid NOT NULL REFERENCES users (user_id),
    product_id uuid NOT NULL REFERENCES products (product_id),
    quantity INT NOT NULL
);
