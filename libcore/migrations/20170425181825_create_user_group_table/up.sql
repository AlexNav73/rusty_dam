CREATE TABLE IF NOT EXISTS user_group (
    id uuid PRIMARY KEY,
    name varchar(255) UNIQUE NOT NULL
)
