CREATE OR REPLACE PROCEDURE pr_update_oauth_token(
    IN p_id_oauth_tokens UUID,
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
        refresh_token = p_refresh_token,
        expiry_date = p_expiry_date
    WHERE
        id_oauth_tokens = p_id_oauth_tokens;
END;
$$;
