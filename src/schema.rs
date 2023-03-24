// @generated automatically by Diesel CLI.

diesel::table! {
    totp_keys (id) {
        id -> Integer,
        taget -> Text,
        secret_key -> Text,
    }
}
