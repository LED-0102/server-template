-- Your SQL goes here
CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email    VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    admin    INT     NOT NULL DEFAULT 0
);

CREATE TABLE to_do
(
    id      SERIAL PRIMARY KEY,
    user_id INTEGER   NOT NULL REFERENCES users (id),
    title   VARCHAR   NOT NULL,
    status  VARCHAR   NOT NULL,
    date    timestamp NOT NULL DEFAULT NOW()
);