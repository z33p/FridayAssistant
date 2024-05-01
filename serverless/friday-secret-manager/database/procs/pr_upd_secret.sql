-- DROP PROCEDURE pr_upd_secret
CREATE OR REPLACE PROCEDURE pr_upd_secret(p_key VARCHAR(255), p_value VARCHAR(255))
AS $$
BEGIN
    UPDATE tb_secrets
    SET "value" = p_value
    WHERE "key" = p_key;
END;
$$ LANGUAGE plpgsql;