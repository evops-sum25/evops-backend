CREATE TABLE users (
    id uuid PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE tags (
    id uuid PRIMARY KEY,
    name text UNIQUE NOT NULL
);

CREATE INDEX idx_tags_name ON tags(id);

CREATE TABLE tags_aliases (
    tag_id uuid REFERENCES tags (id),
    alias text,
    PRIMARY KEY (tag_id, alias)
);

CREATE TABLE events (
    id uuid PRIMARY KEY,
    title text NOT NULL,
    description text NOT NULL,
    author_id uuid NOT NULL REFERENCES users (id),
    with_attendance bool NOT NULL,
    created_at timestamptz NOT NULL,
    modified_at timestamptz NOT NULL
);

CREATE INDEX idx_events_created_at_id ON events(id);

CREATE TABLE images (
    id uuid PRIMARY KEY,
    url text NOT NULL,
    event_id uuid NOT NULL REFERENCES events (id)
);

CREATE TABLE events_tags (
    event_id uuid REFERENCES events (id),
    tag_id uuid REFERENCES tags (id),
    PRIMARY KEY (event_id, tag_id)
);

