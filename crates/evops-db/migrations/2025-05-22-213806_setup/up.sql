CREATE TABLE events (
    id uuid PRIMARY KEY,
    title text NOT NULL,
    description text NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE TABLE tags (
    id uuid PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE event_tags (
    event_id uuid REFERENCES events (id),
    tag_id uuid REFERENCES tags (id),
    PRIMARY KEY (event_id, tag_id)
);
