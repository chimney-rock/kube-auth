table! {
    use diesel::sql_types::*;

    tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        claims -> Jsonb,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
