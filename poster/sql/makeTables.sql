PRAGMA foreign_keys = ON;

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
    content         TEXT    NOT NULL, -- but can be ""
    date            TEXT    NOT NULL, -- unix time, seconds as 0s padded String

    account_id         INTEGER    NOT NULL, -- Many posts : 1 account


    FOREIGN KEY (account_id) REFERENCES accounts(id)
        ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS comments (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    content         TEXT    NOT NULL DEFAULT "", -- but can be ""
    date            TEXT    NOT NULL, -- unix time, seconds as 0s padded String

    account_id      INTEGER NOT NULL,   -- Many comments : 1 account
    post_id         INTEGER NOT NULL,   -- Many comments : 1 account
    parent_comment_id       INTEGER,  -- Many comments : 1 parent comment, if null: top level comment


    FOREIGN KEY (account_id) REFERENCES accounts(id)
        ON DELETE CASCADE,

    FOREIGN KEY (post_id) REFERENCES posts(id)
        ON DELETE CASCADE,
    
    FOREIGN KEY (parent_comment_id) REFERENCES comments(id)
        ON DELETE CASCADE
);