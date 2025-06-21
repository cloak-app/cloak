-- Add up migration script here
CREATE TABLE novel (
    id INTEGER PRIMARY KEY,
    title TEXT,
    path TEXT,
    read_position INTEGER,
    read_progress REAL
);