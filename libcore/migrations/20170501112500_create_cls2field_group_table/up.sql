CREATE TABLE IF NOT EXISTS classification2field_groups (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    classification_id uuid NOT NULL REFERENCES classifications,
    field_group_id uuid NOT NULL REFERENCES field_groups
)
