CREATE TABLE IF NOT EXISTS field_groups (
    id uuid PRIMARY KEY,
    name varchar(255) UNIQUE NOT NULL
)
