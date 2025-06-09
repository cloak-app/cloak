-- Add up migration script here
CREATE TABLE novel (
    id INTEGER PRIMARY KEY,
    title TEXT,
    path TEXT,
    last_read_position INTEGER,
    total_lines INTEGER
);