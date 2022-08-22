table! {
    consumers (id) {
        id -> Integer,
        tel_user_id -> Text,
        username -> Nullable<Text>,
        first_name -> Text,
        last_name -> Nullable<Text>,
        is_bot -> Bool,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
