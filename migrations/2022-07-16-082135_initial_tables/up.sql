CREATE TYPE currency_type AS ENUM('USD', 'IRR', 'CAD', 'Euore');

CREATE TABLE users(
	name text NOT NULL, 
	username text PRIMARY KEY,
	password text NOT NULL
);

CREATE TABLE account(
	balance text DEFAULT '0',
	user_id text NOT NULL,
	id serial PRIMARY KEY,
	name text NOT NULL, 

	FOREIGN KEY (user_id) REFERENCES users (username)
);

CREATE TABLE transaction(
	kind boolean NOT NULL,
	source text,
	note text,
	value text NOT NULL,
	currency currency_type,
	time date NOT NULL,
	user_id text NOT NULL,
	id serial PRIMARY KEY,
	bank_account integer,

	FOREIGN KEY (bank_account) REFERENCES account (id),
	FOREIGN KEY (user_id) REFERENCES users (username)
);

