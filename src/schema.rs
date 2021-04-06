table! {
    employees (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        name -> Text,
        email -> Varchar,
        state -> Text,
        password -> Nullable<Text>,
        bio -> Nullable<Text>,
        image -> Nullable<Bytea>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    users,
);
