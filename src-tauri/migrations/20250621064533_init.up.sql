-- Add up migration script here
CREATE TABLE novel (
    id INTEGER PRIMARY KEY,
    title TEXT,
    path TEXT,
    read_position INTEGER,
    read_progress REAL,
    file_size INTEGER,
    updated_at TEXT DEFAULT (DATETIME('now', 'localtime')),
    created_at TEXT DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER set_updated_at_novel
AFTER UPDATE ON novel
FOR EACH ROW
BEGIN
    UPDATE novel SET updated_at = DATETIME('now', 'localtime') WHERE id = OLD.id;
END;
