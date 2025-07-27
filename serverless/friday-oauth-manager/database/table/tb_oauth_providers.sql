--DROP TABLE tb_oauth_providers;
CREATE TABLE IF NOT EXISTS tb_oauth_providers (
    id_provider INT PRIMARY KEY,
    provider_name varchar(255) NOT NULL,
    client_id text NOT NULL,
    client_secret_value text NOT NULL,
    redirect_uri text NOT NULL
);
