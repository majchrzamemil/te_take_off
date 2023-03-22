-- Add migration script here
CREATE TABLE IF NOT EXISTS te_take_off.users (
    nr_tel text PRIMARY KEY,
    password text NOT NULL,
    email text NOT NULL,
    verified bool NOT NULL DEFAULT FALSE
);

