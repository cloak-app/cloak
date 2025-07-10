-- Add down migration script here
ALTER TABLE novel DROP COLUMN cover;
ALTER TABLE novel DROP COLUMN author;
ALTER TABLE novel DROP COLUMN description;
