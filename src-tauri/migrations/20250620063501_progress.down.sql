-- Add down migration script here
ALTER TABLE novel ADD COLUMN total_lines INTEGER;
ALTER TABLE novel DROP COLUMN read_progress;
