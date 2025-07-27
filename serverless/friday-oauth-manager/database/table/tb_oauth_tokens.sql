CREATE TABLE tb_oauth_tokens (
    id_oauth_tokens UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    expiry_date TIMESTAMP WITH TIME ZONE NOT NULL
);