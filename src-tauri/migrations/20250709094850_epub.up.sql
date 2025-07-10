-- Add up migration script here
ALTER TABLE novel ADD COLUMN cover BLOB;
ALTER TABLE novel ADD COLUMN author TEXT;
ALTER TABLE novel ADD COLUMN description TEXT;