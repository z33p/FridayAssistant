-- DROP PROCEDURE pr_del_secret
CREATE OR REPLACE PROCEDURE pr_del_secret(p_key VARCHAR(255))
AS $$
BEGIN
    DELETE FROM tb_secrets
    WHERE "key" = p_key;
END;
$$ LANGUAGE plpgsql;
