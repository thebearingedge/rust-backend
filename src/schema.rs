table! {
    use diesel::sql_types::*;

    users (user_id) {
        user_id -> Uuid,
        email -> Text,
        name -> Text,
        password -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
