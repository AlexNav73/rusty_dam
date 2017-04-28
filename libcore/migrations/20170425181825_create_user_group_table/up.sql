CREATE TABLE IF NOT EXISTS user_group (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar(255) UNIQUE NOT NULL
)
