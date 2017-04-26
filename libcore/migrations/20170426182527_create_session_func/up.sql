CREATE OR REPLACE FUNCTION create_session(uname varchar(255), upasswd varchar(255)) RETURNS uuid AS $$
DECLARE
    session_id uuid := null;
BEGIN
    INSERT INTO sessions(user_id, login)
    SELECT id, login
    FROM users
    WHERE EXISTS (
        SELECT id FROM users WHERE login = uname AND password = upasswd
    ) RETURNING sessions.id INTO session_id;
    RETURN session_id;
END;
$$ LANGUAGE plpgsql;
