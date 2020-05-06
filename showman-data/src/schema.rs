table! {
    character (character_id) {
        character_id -> Unsigned<Integer>,
        show_id -> Unsigned<Integer>,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    prop (prop_id) {
        prop_id -> Unsigned<Integer>,
        show_id -> Unsigned<Integer>,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    scene (scene_id) {
        scene_id -> Unsigned<Integer>,
        show_id -> Unsigned<Integer>,
        number -> Integer,
        title -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    show (show_id) {
        show_id -> Unsigned<Integer>,
        title -> Varchar,
        description -> Nullable<Text>,
        creation -> Datetime,
    }
}

table! {
    sound (sound_id) {
        sound_id -> Unsigned<Integer>,
        show_id -> Unsigned<Integer>,
        name -> Varchar,
        filename -> Nullable<Varchar>,
    }
}

table! {
    track (track_id) {
        track_id -> Unsigned<Integer>,
        show_id -> Unsigned<Integer>,
        title -> Varchar,
        live -> Bool,
        filename -> Nullable<Varchar>,
    }
}

joinable!(character -> show (show_id));
joinable!(prop -> show (show_id));
joinable!(scene -> show (show_id));
joinable!(sound -> show (show_id));
joinable!(track -> show (show_id));

allow_tables_to_appear_in_same_query!(
    character,
    prop,
    scene,
    show,
    sound,
    track,
);
