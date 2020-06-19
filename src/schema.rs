table! {
    tasks (id) {
        id -> Varchar,
        title -> Varchar,
        completed -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(tasks, users,);
