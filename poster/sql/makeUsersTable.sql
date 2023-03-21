CREATE TABLE IF NOT EXISTS users (
    id              TEXT    PRIMARY KEY, -- = username
    password_hash   TEXT    NOT NULL
);