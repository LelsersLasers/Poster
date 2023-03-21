CREATE TABLE IF NOT EXISTS accounts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name    TEXT    NOT NULL UNIQUE
    -- TODO: 1:1 relationship with users table
);
