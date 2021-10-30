table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        is_active -> Bool,
        is_super -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
