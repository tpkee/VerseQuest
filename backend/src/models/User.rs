use diesel::prelude::*;

#[derive(Debug, PartialEq, Eq, diesel_derive_enum::DbEnum)] // https://github.com/adwhit/diesel-derive-enum
#[ExistingTypePath = "crate::schema::sql_types::UserRoles"]
pub enum UserRoles {
    Admin,
    User,
    Superadmin,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: uuid::Uuid,
    roles: Vec<Option<UserRoles>>,
    username: String,
    email: String,
    password: String,
    biography: String,
    is_verified: bool,
    created_at: std::time::SystemTime,
    updated_at: std::time::SystemTime,
    is_deleted: bool,
    deleted_at: Option<std::time::SystemTime>,
    image_name: String,
    image_url: String,
}
