/*
CREATE TABLE classifications (
	id uuid PRIMARY KEY,
    parent_id uuid REFERENCES classifications(id),
    name varchar(255) NOT NULL CHECK (name <> '')
);

CREATE INDEX parent_id_idx ON classifications(parent_id); 

SELECT * FROM classifications;

*/

-- INSERT INTO classifications VALUES(uuid_generate_v4(), NULL, 'Content LAB')
-- INSERT INTO classifications VALUES(uuid_generate_v4(), '404852a1-af77-4ede-b269-a8cc597ed944', 'Category')
-- INSERT INTO classifications VALUES(uuid_generate_v4(), '9550cf28-3062-4f93-8000-6038ae3e2761', 'Product Images')
-- INSERT INTO classifications VALUES(uuid_generate_v4(), '9550cf28-3062-4f93-8000-6038ae3e2761', 'Brand')
-- INSERT INTO classifications VALUES(uuid_generate_v4(), 'd1d31d83-427c-4a8c-882e-5da3198244f4', 'Armani')

/*
CREATE OR REPLACE FUNCTION get_all_children_array(use_parent uuid) RETURNS uuid[] AS $$
DECLARE
    process_parents uuid[] := ARRAY[use_parent];
    children uuid[] := '{}';
    new_children uuid[];
BEGIN
    WHILE (array_upper(process_parents, 1) IS NOT NULL) LOOP
        new_children := ARRAY(SELECT id FROM classifications WHERE parent_id = ANY(process_parents) AND id <> ALL(children));
        children := children || new_children;
        process_parents := new_children;
    END LOOP;
    RETURN children;
END;
$$ LANGUAGE plpgsql;
*/

/*
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
*/

SELECT get_classification_name_path('0f6c2408-8e68-4c75-a14f-1c84b63868c6');

SELECT * FROM classifications WHERE id = any(get_all_children_array('9550cf28-3062-4f93-8000-6038ae3e2761'));

