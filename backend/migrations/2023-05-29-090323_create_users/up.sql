CREATE TYPE user_role AS ENUM (
    'admin',
    'user',
    'superadmin'
    );

CREATE TABLE IF NOT EXISTS Users (
    id              UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    role            user_role DEFAULT 'user' NOT NULL,
    username        VARCHAR(255) NOT NULL,
    email           VARCHAR(255) NOT NULL,
    password_hash   bytea NOT NULL,
    biography       TEXT NOT NULL DEFAULT '',
    is_verified     BOOLEAN DEFAULT FALSE NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    is_deleted      BOOLEAN DEFAULT FALSE NOT NULL,
    deleted_at      TIMESTAMP,
    image_name      VARCHAR(255) NOT NULL DEFAULT '',
    image_url       VARCHAR(255) NOT NULL DEFAULT ''
    );