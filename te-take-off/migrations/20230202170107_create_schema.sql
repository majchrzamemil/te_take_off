-- Add migration script here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE SCHEMA IF NOT EXISTS te_take_off;

CREATE TABLE IF NOT EXISTS te_take_off.users (
    login text PRIMARY KEY,
    password text NOT NULL,
    verified bool NOT NULL DEFAULT FALSE
);

CREATE TYPE opinion_type AS ENUM (
    'drunk',
    'late',
    'abusive'
);

CREATE TABLE IF NOT EXISTS te_take_off.opinions (
    nr_tel integer NOT NULL,
    opinion_category opinion_type NOT NULL,
    custom_opinion text
);

