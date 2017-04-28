CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    login varchar(255) UNIQUE NOT NULL,
    password varchar(64) NOT NULL,
    email varchar(255)
)
