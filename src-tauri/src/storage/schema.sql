CREATE TABLE attachments (
    id TEXT PRIMARY KEY NOT NULL,
    source TEXT NOT NULL CHECK (source IN ('capture', 'import')),
    name TEXT NOT NULL,
    mime TEXT NOT NULL CHECK (mime IN ('image/png', 'image/jpeg')),
    byte_count INTEGER NOT NULL CHECK (byte_count > 0),
    width INTEGER NOT NULL CHECK (width > 0),
    height INTEGER NOT NULL CHECK (height > 0),
    sha256 TEXT NOT NULL,
    created_at_ms INTEGER NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0 CHECK (deleted IN (0, 1))
);
CREATE INDEX attachments_source ON attachments(source, deleted, created_at_ms);
PRAGMA user_version = 1;
