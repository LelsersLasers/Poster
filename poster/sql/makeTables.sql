CREATE TABLE IF NOT EXISTS users (
    id              TEXT    PRIMARY KEY, -- = username
    password_hash   TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name    TEXT    NOT NULL UNIQUE,

    
    -- 1 user : 1 account
    user_id         TEXT    NOT NULL UNIQUE,
    FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
);