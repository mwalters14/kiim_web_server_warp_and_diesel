// @generated automatically by Diesel CLI.

diesel::table! {
    questions {
        id -> Int4,
        title -> VarChar,
        content -> Text,
        tags -> Nullable<Array<Text>>,
        created_on -> Timestamp,
    }
}
