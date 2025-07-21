CREATE TABLE tb_oauth_tokens (
    id_oauth_tokens UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    access_token VARCHAR(2048) NOT NULL,
    refresh_token VARCHAR(255) NOT NULL,
    expiry_date TIMESTAMP WITH TIME ZONE NOT NULL
);