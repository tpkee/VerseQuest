CREATE TABLE IF NOT EXISTS Chapters (
                                        id             UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                                        book_id        UUID REFERENCES Books (id) NOT NULL,
                                        title          VARCHAR(255) NOT NULL,
                                        summary        TEXT NOT NULL DEFAULT '',
                                        content        TEXT NOT NULL,
                                        total_comments INTEGER DEFAULT 0 NOT NULL,
                                        total_reads    INTEGER DEFAULT 0 NOT NULL,
                                        chapter_number INTEGER DEFAULT 1 NOT NULL,
                                        is_deleted     BOOLEAN DEFAULT FALSE NOT NULL,
                                        deleted_at     TIMESTAMP,
                                        created_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                                        updated_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
