// @generated automatically by Diesel CLI.

diesel::table! {
    event_tags (event_id, tag_id) {
        event_id -> Int8,
        tag_id -> Int8,
    }
}

diesel::table! {
    events (id) {
        id -> Int8,
        title -> Text,
        description -> Text,
        author -> Int8,
        location_id -> Nullable<Int8>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    locations (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        email_address -> Text,
    }
}

diesel::joinable!(event_tags -> events (event_id));
diesel::joinable!(event_tags -> tags (tag_id));
diesel::joinable!(events -> locations (location_id));
diesel::joinable!(events -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    event_tags,
    events,
    locations,
    tags,
    users,
);
