CREATE TYPE currency_type AS ENUM('USD', 'IRR', 'CAD', 'Euore');

CREATE TABLE users(
	name text NOT NULL, 
	username text PRIMARY KEY,
	password text NOT NULL,
	api_token text NOT NULL,
	role boolean NOT NULL DEFAULT 'f',

	UNIQUE(api_token)
);

CREATE TABLE account(
	balance text NOT NULL DEFAULT '0',
	user_id text NOT NULL,
	id serial PRIMARY KEY,
	name text NOT NULL, 
	
	UNIQUE (user_id, name),
	FOREIGN KEY (user_id) REFERENCES users (username)
);

CREATE TABLE transaction(
	kind boolean NOT NULL,
	title text,
	value text NOT NULL,
	currency currency_type DEFAULT 'USD' NOT NULL,
	time date NOT NULL,
	user_id text NOT NULL,
	id serial PRIMARY KEY,
	bank_account integer NOT NULL,

	FOREIGN KEY (bank_account) REFERENCES account (id),
	FOREIGN KEY (user_id) REFERENCES users (username)
);

