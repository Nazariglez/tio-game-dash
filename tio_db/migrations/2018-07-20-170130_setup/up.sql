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

CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    developer_id INTEGER REFERENCES developers(id) NOT NULL,
    name VARCHAR(255) UNIQUE NOT NULL,
    url VARCHAR(255) UNIQUE NOT NULL, 
    state SMALLINT NOT NULL DEFAULT 0,
    description TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- password: admin
INSERT INTO administrators 
    (email, password, created_at, updated_at) 
    VALUES ('info@tarentola.io', '$2a$12$WUUZjgvQKQsWtuDnttAav.GIdUbLptFuNrw4TtwW.BnMQFSiFjNH6', current_timestamp, current_timestamp);

-- password: admin
INSERT INTO developers 
    (email, password, created_at, updated_at)
    VALUES ('info+dev@tarentola.io', '$2a$12$WUUZjgvQKQsWtuDnttAav.GIdUbLptFuNrw4TtwW.BnMQFSiFjNH6', current_timestamp, current_timestamp);