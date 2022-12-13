-- Add migration script here
CREATE TABLE categories(
    category_id uuid PRIMARY KEY,
    name TEXT NOT NULL
);
