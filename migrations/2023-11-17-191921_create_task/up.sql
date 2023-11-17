-- Your SQL goes here

CREATE TABLE tasks (
    id INT PRIMARY KEY,
    task TEXT,
    created DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed DATETIME NULL
)