-- Add down migration script here
ALTER TABLE novel DROP COLUMN is_open;
DROP TRIGGER ensure_single_open_novel;
