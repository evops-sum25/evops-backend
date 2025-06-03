// @generated automatically by Diesel CLI.

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
    users (id) {
        id -> Int8,
        email_address -> Text,
    }
}

diesel::joinable!(events -> locations (location_id));
diesel::joinable!(events -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    locations,
    users,
);
