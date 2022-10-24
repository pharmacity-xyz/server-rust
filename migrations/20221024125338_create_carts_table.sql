-- Add migration script here
CREATE TABLE carts (
    user_id uuid NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCEs users(id),
    product_id uuid NOT NULL,
    CONSTRAINT fk_product
        FOREIGN KEY(product_id)
            REFERENCEs products(id),
    quantity INT NOT NULL
);