// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "book_genre"))]
    pub struct BookGenre;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "book_language"))]
    pub struct BookLanguage;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "book_status"))]
    pub struct BookStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BookStatus;
    use super::sql_types::BookLanguage;
    use super::sql_types::BookGenre;

    books (id) {
        id -> Uuid,
        author_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        image_name -> Varchar,
        #[max_length = 255]
        image_url -> Varchar,
        summary -> Text,
        status -> BookStatus,
        language -> BookLanguage,
        genre -> BookGenre,
        total_chapters -> Int4,
        tags -> Array<Nullable<Text>>,
        is_nsfw -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
        removal_reason -> Text,
    }
}

diesel::table! {
    chapters (id) {
        id -> Uuid,
        book_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        summary -> Text,
        content -> Text,
        total_comments -> Int4,
        total_reads -> Int4,
        chapter_number -> Int4,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        chapter_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        removed_at -> Nullable<Timestamp>,
        is_deleted -> Bool,
        removal_reason -> Text,
    }
}

diesel::table! {
    reviews (id) {
        id -> Uuid,
        book_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        is_recommended -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        is_deleted -> Bool,
        removal_reason -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Uuid,
        role -> UserRole,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Bytea,
        biography -> Text,
        is_verified -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
        #[max_length = 255]
        image_name -> Varchar,
        #[max_length = 255]
        image_url -> Varchar,
    }
}

diesel::joinable!(books -> users (author_id));
diesel::joinable!(chapters -> books (book_id));
diesel::joinable!(comments -> chapters (chapter_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(reviews -> books (book_id));
diesel::joinable!(reviews -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    chapters,
    comments,
    reviews,
    users,
);
