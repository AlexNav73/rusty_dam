CREATE TABLE IF NOT EXISTS user2user_group (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL REFERENCES users,
    user_group_id uuid NOT NULL REFERENCES user_group
)
