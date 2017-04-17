/*
DO $$
DECLARE lastid uuid;
DECLARE category_id uuid;
BEGIN
  INSERT INTO classifications VALUES(uuid_generate_v4(), NULL, 'Content LAB') 
  RETURNING id INTO lastid;

  INSERT INTO classifications VALUES(uuid_generate_v4(), lastid, 'Category')
  RETURNING id INTO category_id;
  
  INSERT INTO classifications VALUES(uuid_generate_v4(), category_id, 'Product Images')
  RETURNING id INTO lastid;
  
  INSERT INTO classifications VALUES(uuid_generate_v4(), category_id, 'Brand')
  RETURNING id INTO lastid;
  
  INSERT INTO classifications VALUES(uuid_generate_v4(), lastid, 'Armani');
END $$;
*/

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

SELECT * FROM classifications WHERE id = any(get_all_children_array('9550cf28-3062-4f93-8000-6038ae3e2761'));

