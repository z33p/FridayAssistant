-- DROP FUNCTION fn_get_first_oauth_token_by_refresh_token;
CREATE OR REPLACE FUNCTION fn_get_first_oauth_tokens_by_last_expiry_date()
RETURNS TABLE (
    id_oauth_tokens UUID,
    access_token VARCHAR(2048),
    refresh_token VARCHAR(255),
    expiry_date TIMESTAMP WITH TIME ZONE
)
AS $$
BEGIN
    RETURN QUERY
    SELECT
        tb_oauth_tokens.id_oauth_tokens,
        tb_oauth_tokens.access_token,
        tb_oauth_tokens.refresh_token,
        tb_oauth_tokens.expiry_date
    FROM
        tb_oauth_tokens
    ORDER BY
        tb_oauth_tokens.expiry_date DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;
