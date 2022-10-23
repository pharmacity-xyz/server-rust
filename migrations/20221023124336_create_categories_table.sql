-- Add migration script here
CREATE TABLE categories(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL
);
