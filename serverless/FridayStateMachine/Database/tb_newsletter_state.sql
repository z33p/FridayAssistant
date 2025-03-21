--DROP TABLE tb_newsletter_state;
CREATE TABLE IF NOT EXISTS tb_newsletter_state (
    correlation_id UUID PRIMARY KEY,
    payload json NOT NULL,
    previous_state VARCHAR(64) NOT NULL,
    current_state VARCHAR(64) NOT NULL
);
SELECT *
FROM tb_newsletter_state;
DELETE FROM tb_newsletter_state;
INSERT INTO tb_newsletter_state (
        correlation_id,
        payload,
        previous_state,
        current_state
    )
VALUES (
        '85a30000-47e7-0ccc-231e-08dd5f6297b5',
        '{}',
        'None',
        'None'
    );