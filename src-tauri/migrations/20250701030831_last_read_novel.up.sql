ALTER TABLE novel ADD COLUMN is_open INTEGER DEFAULT 0;

CREATE TRIGGER ensure_single_open_novel
BEFORE UPDATE ON novel
FOR EACH ROW
BEGIN
    UPDATE novel SET is_open = 0 WHERE id != NEW.id;
END;
