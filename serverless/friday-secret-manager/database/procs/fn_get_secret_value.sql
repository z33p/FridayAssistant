-- DROP FUNCTION fn_get_secret_value;
CREATE OR REPLACE FUNCTION fn_get_secret_value(p_key VARCHAR(255))
RETURNS VARCHAR(255) AS $$
DECLARE
    secret_value VARCHAR(255);
BEGIN
    SELECT "value" INTO secret_value
    FROM tb_secrets
    WHERE "key" = p_key;
    
    RETURN secret_value;
END;
$$ LANGUAGE plpgsql;