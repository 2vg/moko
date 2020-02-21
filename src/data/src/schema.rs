#[macro_use]
table! {
    files (id) {
        id -> Integer,
        file_name -> Text,
        is_no_expires -> Bool,
        expires -> Text,
        key -> Text,
    }
}
