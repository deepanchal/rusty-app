// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        invalidated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Int4>,
        updated_by -> Nullable<Int4>,
        first_name -> Varchar,
        last_name -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    todo (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        invalidated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Int4>,
        updated_by -> Nullable<Int4>,
        title -> Varchar,
        description -> Nullable<Varchar>,
        done -> Bool,
        user_id -> Int4,
    }
}

diesel::table! {
    user_account (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        invalidated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Int4>,
        updated_by -> Nullable<Int4>,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    person,
    todo,
    user_account,
);
