CREATE TABLE IF NOT EXISTS field2field_groups (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    field_id uuid NOT NULL REFERENCES fields,
    field_group_id uuid NOT NULL REFERENCES field_groups
)
