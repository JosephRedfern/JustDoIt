-- Your SQL goes here

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    body TEXT,
    created DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed DATETIME NULL
)