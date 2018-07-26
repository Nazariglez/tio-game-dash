-- Your SQL goes here
CREATE TABLE administrators (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    level SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE developers (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE apps (
    id SERIAL PRIMARY KEY,
    developer_id INTEGER REFERENCES developers(id),
    name VARCHAR(255) UNIQUE NOT NULL,
    url VARCHAR(255) UNIQUE NOT NULL, 
    state SMALLINT NOT NULL DEFAULT 0, -- 0:active, 1:to_review, 2:disabled, 3:banned
    description TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE keys (  -- todo add field to check rate limits?
    id SERIAL PRIMARY KEY,
    app_id INTEGER REFERENCES apps(id),
    live BOOLEAN NOT NULL DEFAULT false,    --live or test "pk_live_uuid or pk_test_uuid"
    public CHAR(36) UNIQUE NOT NULL,        --pk
    secret  CHAR(36) UNIQUE NOT NULL,       --sk
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE apps_categories (
    id SERIAL PRIMARY KEY,
    app_id INTEGER REFERENCES apps(id) NOT NULL,
    category_id INTEGER REFERENCES categories(id) NOT NULL
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE apps_tags (
    id SERIAL PRIMARY KEY,
    app_id INTEGER REFERENCES apps(id) NOT NULL,
    tag_id INTEGER REFERENCES tags(id) NOT NULL
);

-- password: admin
INSERT INTO administrators 
    (email, password, created_at, updated_at) 
    VALUES ('info@tarentola.io', '$2a$12$WUUZjgvQKQsWtuDnttAav.GIdUbLptFuNrw4TtwW.BnMQFSiFjNH6', current_timestamp, current_timestamp);

-- password: admin
INSERT INTO developers 
    (email, password, created_at, updated_at)
    VALUES ('info+dev@tarentola.io', '$2a$12$WUUZjgvQKQsWtuDnttAav.GIdUbLptFuNrw4TtwW.BnMQFSiFjNH6', current_timestamp, current_timestamp);