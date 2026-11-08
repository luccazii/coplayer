ALTER TABLE chats ADD COLUMN draft_generation INTEGER NOT NULL DEFAULT 0;
CREATE TABLE messages (
    id TEXT PRIMARY KEY NOT NULL,
    campaign_id TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    configuration_id TEXT NOT NULL,
    draft_generation INTEGER NOT NULL,
    text TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status = 'local_only'),
    created_at_ms INTEGER NOT NULL,
    UNIQUE (chat_id, draft_generation),
    FOREIGN KEY (campaign_id, chat_id) REFERENCES chats(campaign_id, id),
    FOREIGN KEY (campaign_id, configuration_id) REFERENCES campaign_configurations(campaign_id, id)
);
ALTER TABLE attachments ADD COLUMN owner_chat_id TEXT REFERENCES chats(id);
ALTER TABLE attachments ADD COLUMN message_id TEXT REFERENCES messages(id);
CREATE INDEX attachments_chat ON attachments(owner_chat_id, message_id, deleted, created_at_ms);
CREATE INDEX messages_chat ON messages(chat_id, created_at_ms);
CREATE TRIGGER attachment_message_owner BEFORE UPDATE OF owner_chat_id, message_id ON attachments
WHEN NEW.message_id IS NOT NULL AND NOT EXISTS (
    SELECT 1 FROM messages WHERE id=NEW.message_id AND chat_id=NEW.owner_chat_id
)
BEGIN SELECT RAISE(ABORT, 'Message and attachment must belong to the same chat'); END;
PRAGMA user_version = 3;
