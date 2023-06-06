use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq, DbEnum, Serialize, Deserialize)] // https://github.com/adwhit/diesel-derive-enum
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    admin,
    user,
    superadmin,
}

#[derive(Serialize, Selectable, Queryable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    id: uuid::Uuid,
    role: UserRole,
    username: String,
    email: String,
    password_hash: Vec<u8>,
    biography: String,
    is_verified: bool,
    created_at: NaiveDateTime, // ? NaiveDateTime expects an ISO string WITHOUT the timezone (es: 1970-01-01T00:00:00). To check it out, use NaiveDateTime::default()
    updated_at: NaiveDateTime,
    is_deleted: bool,
    deleted_at: Option<NaiveDateTime>,
    image_name: String,
    image_url: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub role: UserRole,
    pub username: String,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub biography: String,
    pub is_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub image_name: String,
    pub image_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PayloadUser {
    pub role: UserRole,
    pub username: String,
    pub email: String,
    pub password: String,
    pub biography: String,
    pub is_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub image_name: String,
    pub image_url: String,
}