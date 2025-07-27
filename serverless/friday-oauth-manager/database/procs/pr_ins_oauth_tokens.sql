-- DROP PROCEDURE pr_ins_oauth_tokens
CREATE OR REPLACE PROCEDURE pr_ins_oauth_tokens(
        IN p_access_token TEXT,
        IN p_refresh_token TEXT,
        IN p_expiry_date TIMESTAMP WITH TIME ZONE
    ) LANGUAGE plpgsql AS $$ BEGIN
INSERT INTO tb_oauth_tokens (access_token, refresh_token, expiry_date)
VALUES (p_access_token, p_refresh_token, p_expiry_date);
END;
$$;