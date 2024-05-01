-- DROP FUNCTION fn_get_all_secrets;
CREATE OR REPLACE FUNCTION fn_get_all_secrets()
RETURNS TABLE ("key" VARCHAR(255), "value" VARCHAR(255)) AS $$
BEGIN
    RETURN QUERY
    SELECT s."key", s."value"
    FROM tb_secrets s;
END;
$$ LANGUAGE plpgsql;
