CREATE OR REPLACE FUNCTION create_admin(uname varchar(255), upasswd varchar(255)) RETURNS uuid AS $$
DECLARE
    admin_user_group uuid := null;
    admin_user_id uuid := null;
BEGIN
    SELECT id::uuid
    INTO admin_user_group
    FROM user_group
    WHERE name = 'administrators';
    
    IF admin_user_group IS NULL THEN
    	INSERT INTO user_group(name) VALUES('administrators')
        RETURNING id INTO admin_user_group;
    END IF;
    
    IF (SELECT id FROM users WHERE login = uname) IS NULL THEN
    	INSERT INTO users(login, password) VALUES(uname, upasswd)
    	RETURNING id INTO admin_user_id;
        
        INSERT INTO user2user_group(user_id, user_group_id) 
        VALUES(admin_user_id, admin_user_group);
        
        RETURN admin_user_id;
    END IF;
    
    RAISE EXCEPTION 'User with name [%] already exists', uname;
END;
$$ LANGUAGE plpgsql;
