CREATE OR REPLACE FUNCTION get_classification_name_path(cls uuid) RETURNS varchar(255)[] AS $$
DECLARE
    name_path varchar(255)[] := '{}';
    curr_node uuid := cls;
    curr_node_name varchar(255);
BEGIN
    WHILE (curr_node IS NOT NULL) LOOP
        SELECT parent_id::uuid, name::varchar 
        INTO curr_node, curr_node_name 
        FROM classifications 
        WHERE id = curr_node::uuid;
        name_path := curr_node_name || name_path;
    END LOOP;
    RETURN name_path;
END;
$$ LANGUAGE plpgsql;
