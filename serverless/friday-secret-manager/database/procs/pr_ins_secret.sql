-- DROP PROCEDURE pr_ins_secret
CREATE OR REPLACE PROCEDURE pr_ins_secret(p_key VARCHAR(255), p_value VARCHAR(255))
AS $$
BEGIN
    INSERT INTO tb_secrets ("key", "value")
    VALUES (p_key, p_value);
END;
$$ LANGUAGE plpgsql;