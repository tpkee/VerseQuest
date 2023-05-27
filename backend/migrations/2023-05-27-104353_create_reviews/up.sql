CREATE TABLE IF NOT EXISTS Reviews (
                                       id             UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                                       book_id        UUID REFERENCES Books (id) NOT NULL,
                                       user_id        UUID REFERENCES Users (id) NOT NULL,
                                       title          VARCHAR(255) NOT NULL,
                                       content        TEXT NOT NULL,
                                       is_recommended BOOLEAN NOT NULL,
                                       created_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                                       updated_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                                       deleted_at     TIMESTAMP,
                                       is_deleted     BOOLEAN DEFAULT FALSE NOT NULL,
                                       removal_reason TEXT NOT NULL DEFAULT ''
);
