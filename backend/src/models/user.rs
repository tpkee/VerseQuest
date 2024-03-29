use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Debug, PartialEq, Eq, DbEnum, serde::Serialize)] // https://github.com/adwhit/diesel-derive-enum
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    Admin,
    User,
    Superadmin,
}

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    id: uuid::Uuid,
    role: Vec<Option<UserRole>>,
    username: String,
    email: String,
    password_hash: String,
    biography: String,
    is_verified: bool,
    created_at: std::time::SystemTime,
    updated_at: std::time::SystemTime,
    is_deleted: bool,
    deleted_at: Option<std::time::SystemTime>,
    image_name: String,
    image_url: String,
}