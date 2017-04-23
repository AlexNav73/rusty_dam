CREATE TABLE IF NOT EXISTS fields (
    id uuid PRIMARY KEY,
    name varchar(255) UNIQUE NOT NULL
)
