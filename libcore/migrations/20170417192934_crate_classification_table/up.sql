CREATE EXTENSION "uuid-ossp";
CREATE TABLE IF NOT EXISTS classifications (
	id uuid PRIMARY KEY,
    parent_id uuid REFERENCES classifications(id),
    name varchar(255) NOT NULL CHECK (name <> '')
);
