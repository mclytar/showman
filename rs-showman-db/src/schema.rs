table! {
    authentication (auth_id) {
        auth_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        method -> Varchar,
        user_data -> Varchar,
        token -> Text,
    }
}

table! {
    session (token) {
        token -> Varchar,
        user_id -> Unsigned<Integer>,
        expiration -> Datetime,
    }
}

table! {
    show (show_id) {
        show_id -> Unsigned<Integer>,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        description -> Nullable<Text>,
        creation -> Datetime,
    }
}

table! {
    user (user_id) {
        user_id -> Unsigned<Integer>,
        name -> Varchar,
        surname -> Varchar,
        role -> Varchar,
    }
}

joinable!(authentication -> user (user_id));
joinable!(session -> user (user_id));

allow_tables_to_appear_in_same_query!(
    authentication,
    session,
    show,
    user,
);
