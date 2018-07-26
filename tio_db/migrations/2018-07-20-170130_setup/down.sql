-- This file should undo anything in `up.sql`
DROP TABLE administrators;
DROP TABLE developers CASCADE;
DROP TABLE users;
DROP TABLE apps;
DROP TABLE keys;
DROP TABLE categories;
DROP TABLE apps_categories;
DROP TABLE tags;
DROP TABLE apps_tags;