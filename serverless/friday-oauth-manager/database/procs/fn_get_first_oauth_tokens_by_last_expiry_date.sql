-- DROP FUNCTION fn_get_first_oauth_tokens_by_last_expiry_date;
CREATE OR REPLACE FUNCTION fn_get_first_oauth_tokens_by_last_expiry_date() RETURNS TABLE (
        id_oauth_tokens UUID,
        id_provider INT,
        access_token TEXT,
        refresh_token TEXT,
        expiry_date TIMESTAMP WITH TIME ZONE
    ) AS $$ BEGIN RETURN QUERY
SELECT tb_oauth_tokens.id_oauth_tokens,
    tb_oauth_tokens.id_provider,
    tb_oauth_tokens.access_token,
    tb_oauth_tokens.refresh_token,
    tb_oauth_tokens.expiry_date
FROM tb_oauth_tokens
WHERE tb_oauth_tokens.expiry_date > NOW() -- Somente tokens que ainda não expiraram
ORDER BY tb_oauth_tokens.expiry_date DESC -- O token que expira mais tarde (mais recente)
LIMIT 1;
END;
$$ LANGUAGE plpgsql;