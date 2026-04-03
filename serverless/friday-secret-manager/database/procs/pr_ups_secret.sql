-- DROP PROCEDURE pr_ups_secret
CREATE OR REPLACE PROCEDURE pr_ups_secret(p_key VARCHAR(255), p_value VARCHAR(255)) AS $$ BEGIN
INSERT INTO tb_secrets ("key", "value")
VALUES (p_key, p_value) ON CONFLICT ("key") DO
UPDATE
SET "value" = p_value;
END;
$$ LANGUAGE plpgsql;