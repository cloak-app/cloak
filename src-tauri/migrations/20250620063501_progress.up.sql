-- Add up migration script here
ALTER TABLE novel DROP COLUMN total_lines;
ALTER TABLE novel ADD COLUMN read_progress REAL;