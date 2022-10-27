-- Add migration script here
CREATE TABLE products(
    id TEXT NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT NOT NULL,
    stock INT NOT NULL,
    price INT NOT NULL,
	featured BOOLEAN NOT NULL,
    category_id uuid NOT NULL,
    CONSTRAINT fk_category
        FOREIGN KEY(category_id)
            REFERENCEs categories(id)
);
