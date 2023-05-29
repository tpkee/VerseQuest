CREATE TYPE book_status AS ENUM (
    'ongoing',
    'completed',
    'onhold',
    'dropped'
    );

CREATE TYPE book_language AS ENUM (
    'en', -- English
    'es', -- Spanish
    'fr', -- French
    'de', -- German
    'zh', -- Chinese
    'hi', -- Hindi
    'ar', -- Arabic
    'ru', -- Russian
    'pt', -- Portuguese
    'ja', -- Japanese
    'tr', -- Turkish
    'it', -- Italian
    'th', -- Thai
    'ko', -- Korean
    'pl', -- Polish
    'nl' -- Dutch
    );

CREATE TYPE book_genre AS ENUM (
    'fiction',
    'mystery',
    'thriller',
    'science_fiction',
    'fantasy',
    'romance',
    'historical',
    'biography',
    'horror',
    'crime',
    'young_adult',
    'poetry',
    'drama',
    'comedy',
    'action',
    'adventure'
    );

CREATE TABLE IF NOT EXISTS Books (
    id             UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    author_id      UUID UNIQUE REFERENCES Users (id) NOT NULL,
    title          VARCHAR(255) NOT NULL,
    image_name     VARCHAR(255) NOT NULL DEFAULT '',
    image_url      VARCHAR(255) NOT NULL DEFAULT '',
    summary        TEXT NOT NULL DEFAULT '',
    status         book_status DEFAULT 'ongoing' NOT NULL,
    language       book_language NOT NULL,
    genre          book_genre NOT NULL,
    total_chapters INTEGER DEFAULT 0 NOT NULL,
    tags           TEXT[] NOT NULL,
    is_nsfw        BOOLEAN DEFAULT FALSE NOT NULL,
    created_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    is_deleted     BOOLEAN DEFAULT FALSE NOT NULL,
    deleted_at     TIMESTAMP,
    removal_reason TEXT NOT NULL DEFAULT ''
    );
