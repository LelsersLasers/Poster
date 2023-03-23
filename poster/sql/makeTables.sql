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


CREATE TABLE IF NOT EXISTS posts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    title           TEXT    NOT NULL,
    content         TEXT    NOT NULL,
    date            TEXT    NOT NULL, -- unix time, seconds as String

    -- Many posts : 1 account
    account_id         INTEGER    NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
        ON DELETE CASCADE
);