-- Add migration script here
CREATE TABLE products(
    product_id TEXT PRIMARY KEY,
    product_name TEXT NOT NULL,
    product_description TEXT NOT NULL,
    image_url TEXT NOT NULL,
    stock INT NOT NULL,
    price NUMERIC NOT NULL,
	featured BOOLEAN NOT NULL,
    category_id uuid REFERENCES categories (category_id)
);
