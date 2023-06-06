use argon2::{
    Argon2,
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
};
use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use crate::models::user::{User, NewUser, PayloadUser};
use diesel::prelude::*;
use crate::api::SanitisedBody;
use crate::internal_error;

pub async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    SanitisedBody(Json(body)): SanitisedBody<Json<PayloadUser>>
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string()
        .as_bytes()
        .to_vec();

    // decription assert!(Argon2::default().verify_password("Hello world".as_bytes(), &PasswordHash::new(std::str::from_utf8(&password_hash).unwrap()).unwrap()).is_ok());

    let new_user = NewUser {
        role: body.role,
        username: body.username,
        email: body.email,
        password_hash: password_hash.clone(),
        biography: body.biography,
        is_verified: body.is_verified,
        created_at: body.created_at,
        updated_at: body.updated_at,
        image_name: body.image_name,
        image_url: body.image_url,
    };

    let res = conn
        .interact(|conn| {
            diesel::insert_into(crate::schema::users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}
