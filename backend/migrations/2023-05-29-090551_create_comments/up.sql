CREATE TABLE IF NOT EXISTS Comments (
    id             UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    chapter_id     UUID REFERENCES Chapters (id) NOT NULL,
    user_id        UUID REFERENCES Users (id) NOT NULL,
    content        TEXT NOT NULL,
    created_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    removed_at     TIMESTAMP,
    is_deleted     BOOLEAN NOT NULL DEFAULT FALSE,
    removal_reason TEXT NOT NULL DEFAULT ''
);