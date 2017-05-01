CREATE OR REPLACE FUNCTION get_id_by_path(name_path varchar(255)[]) RETURNS uuid AS $$
DECLARE
    same bool := false;
    result_id uuid := null;
    next_cls uuid;
BEGIN
	SELECT parent_id::uuid, id::uuid 
    INTO next_cls, result_id
    FROM classifications 
    WHERE name = name_path[array_upper(name_path, 1)];

    IF next_cls IS NULL THEN
    	RETURN result_id;
    END IF;

    FOR i IN REVERSE (array_upper(name_path, 1) - 1)..1 LOOP
        SELECT (name = name_path[i])::bool, parent_id::uuid 
        INTO same, next_cls
        FROM classifications
        WHERE id = next_cls;
        
        IF same = false THEN 
        	RETURN null;
		END IF;
    END LOOP;
    
    RETURN result_id;
END;
$$ LANGUAGE plpgsql;
