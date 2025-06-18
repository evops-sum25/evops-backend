// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        author_id -> Uuid,
        with_attendance -> Bool,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
    }
}

diesel::table! {
    events_tags (event_id, tag_id) {
        event_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    images (id) {
        id -> Uuid,
        url -> Text,
        event_id -> Uuid,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    tags_aliases (tag_id, alias) {
        tag_id -> Uuid,
        alias -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        profile_picture_url -> Nullable<Text>,
    }
}

diesel::joinable!(events -> users (author_id));
diesel::joinable!(events_tags -> events (event_id));
diesel::joinable!(events_tags -> tags (tag_id));
diesel::joinable!(images -> events (event_id));
diesel::joinable!(tags_aliases -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    events_tags,
    images,
    tags,
    tags_aliases,
    users,
);
