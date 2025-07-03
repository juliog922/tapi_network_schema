-- Add up migration script here
CREATE TABLE IF NOT EXISTS devices (
    ip VARCHAR PRIMARY KEY,
    port INTEGER,
    auth_type VARCHAR NOT NULL,
    CONSTRAINT check_auth_type CHECK (auth_type IN ('token', 'basic')),
    auth_body VARCHAR NOT NULL,
    auth_uri VARCHAR
);