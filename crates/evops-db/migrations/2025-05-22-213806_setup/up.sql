CREATE TABLE users (
    id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    email_address text UNIQUE NOT NULL
);

CREATE TABLE locations (
    id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL
);

CREATE TABLE events (
    id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title text NOT NULL,
    description text NOT NULL,
    author bigint NOT NULL REFERENCES users (id),
    location_id bigint REFERENCES locations (id),
    created_at timestamptz NOT NULL
);
