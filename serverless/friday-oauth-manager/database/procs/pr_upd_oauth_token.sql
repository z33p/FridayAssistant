-- DROP PROCEDURE pr_upd_oauth_tokens_by_refresh_token
CREATE OR REPLACE PROCEDURE pr_upd_oauth_tokens_by_refresh_token(
    IN p_access_token VARCHAR(2048),
    IN p_refresh_token VARCHAR(255),
    IN p_expiry_date TIMESTAMP WITH TIME ZONE
)
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE tb_oauth_tokens
    SET
        access_token = p_access_token,
        expiry_date = p_expiry_date
    WHERE
        refresh_token = p_refresh_token;
END;
$$;
