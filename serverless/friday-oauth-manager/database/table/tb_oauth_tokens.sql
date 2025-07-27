
CREATE TABLE IF NOT EXISTS tb_oauth_tokens (
    id_oauth_tokens UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    id_provider INT NOT NULL REFERENCES tb_oauth_providers(id_provider),
    access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    expiry_date TIMESTAMP WITH TIME ZONE NOT NULL
);
