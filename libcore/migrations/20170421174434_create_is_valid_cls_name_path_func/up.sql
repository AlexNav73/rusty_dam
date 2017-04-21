CREATE OR REPLACE FUNCTION is_valid_classification_name_path(name_path varchar(255)[]) RETURNS bool AS $$
DECLARE
    same bool := false;
    next_cls uuid;
BEGIN
	SELECT parent_id::uuid 
    INTO next_cls 
    FROM classifications 
    WHERE name = name_path[array_upper(name_path, 1)];

    IF next_cls IS NULL THEN
    	RETURN false;
    END IF;

    FOR i IN REVERSE (array_upper(name_path, 1) - 1)..1 LOOP
        SELECT (name = name_path[i])::bool, parent_id::uuid 
        INTO same, next_cls
        FROM classifications
        WHERE id = next_cls;
        
        IF same = false THEN 
        	RETURN false;
		END IF;
    END LOOP;
    RETURN true;
END;
$$ LANGUAGE plpgsql;
