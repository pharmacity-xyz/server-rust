-- Add migration script here
CREATE TABLE order_items (
      order_id uuid REFERENCES orders(order_id),
    product_id TEXT REFERENCES products(product_id),
    quantity INT NOT NULL,
    total_price NUMERIC NOT NULL  
);
