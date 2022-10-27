-- Add migration script here
CREATE TABLE carts (
	id uuid NOT NULL,
    user_id TEXT NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCEs users(id),
    product_id TEXT NOT NULL,
    CONSTRAINT fk_product
        FOREIGN KEY(product_id)
            REFERENCEs products(id),
    quantity INT NOT NULL
);
