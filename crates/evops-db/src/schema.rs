// @generated automatically by Diesel CLI.

diesel::table! {
    event_tags (event_id, tag_id) {
        event_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    events (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::joinable!(event_tags -> events (event_id));
diesel::joinable!(event_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    event_tags,
    events,
    tags,
);
