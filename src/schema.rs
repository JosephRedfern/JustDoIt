// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Nullable<Integer>,
        task -> Nullable<Text>,
        created -> Nullable<Timestamp>,
        completed -> Nullable<Timestamp>,
    }
}
