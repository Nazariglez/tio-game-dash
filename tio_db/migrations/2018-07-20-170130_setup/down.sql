-- This file should undo anything in `up.sql`
DROP TABLE administrators CASCADE;
DROP TABLE administrator_sessions;

DROP TABLE developers CASCADE;
DROP TABLE developer_sessions;

DROP TABLE users CASCADE;
DROP TABLE user_sessions;

DROP TABLE apps CASCADE;
DROP TABLE keys;

DROP TABLE categories CASCADE;
DROP TABLE apps_categories;

DROP TABLE tags CASCADE;
DROP TABLE apps_tags;