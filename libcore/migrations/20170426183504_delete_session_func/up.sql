CREATE OR REPLACE FUNCTION delete_session(sid uuid) RETURNS bool AS $$
BEGIN
    IF EXISTS (SELECT * FROM sessions WHERE id = sid) THEN
    	DELETE FROM sessions WHERE id = sid;
        RETURN true;
    END IF;
    RETURN false;
END;
$$ LANGUAGE plpgsql;-- Your SQL goes here
