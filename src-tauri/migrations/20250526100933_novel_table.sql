CREATE TABLE chapter (
    id INTEGER PRIMARY KEY,
    novel_id INTEGER,
    title TEXT,
    start_position INTEGER,
);

CREATE TABLE novel (
    id INTEGER PRIMARY KEY,
    title TEXT,
    path TEXT,
    last_read_position INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);