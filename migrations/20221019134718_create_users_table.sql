-- Add migration script here
CREATE TABLE users(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE, 
	password TEXT NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	city TEXT NOT NULL,
	country TEXT NOT NULL,
	company_name TEXT NOT NULL,
	role TEXT NOT NULL
);
