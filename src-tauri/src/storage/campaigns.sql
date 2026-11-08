CREATE TABLE campaigns (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    created_at_ms INTEGER NOT NULL
);
CREATE TABLE campaign_configurations (
    id TEXT PRIMARY KEY NOT NULL,
    campaign_id TEXT NOT NULL REFERENCES campaigns(id),
    game_version TEXT NOT NULL CHECK (length(trim(game_version)) > 0),
    active_dlcs TEXT NOT NULL,
    dlcs_confirmed INTEGER NOT NULL CHECK (dlcs_confirmed = 1),
    no_dlcs_confirmed INTEGER NOT NULL CHECK (no_dlcs_confirmed IN (0, 1)),
    source TEXT NOT NULL CHECK (source = 'player'),
    created_at_ms INTEGER NOT NULL,
    UNIQUE (campaign_id, id)
);
CREATE TRIGGER immutable_configuration BEFORE UPDATE ON campaign_configurations
BEGIN SELECT RAISE(ABORT, 'Historical configurations are immutable'); END;
CREATE TABLE chats (
    id TEXT PRIMARY KEY NOT NULL,
    campaign_id TEXT NOT NULL REFERENCES campaigns(id),
    configuration_id TEXT NOT NULL,
    title TEXT NOT NULL CHECK (length(trim(title)) > 0),
    draft TEXT NOT NULL DEFAULT '',
    revision INTEGER NOT NULL DEFAULT 0,
    created_at_ms INTEGER NOT NULL,
    UNIQUE (campaign_id, id),
    FOREIGN KEY (campaign_id, configuration_id) REFERENCES campaign_configurations(campaign_id, id)
);
CREATE TABLE local_selection (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    campaign_id TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    FOREIGN KEY (campaign_id, chat_id) REFERENCES chats(campaign_id, id)
);
CREATE INDEX chats_campaign ON chats(campaign_id, created_at_ms);
PRAGMA user_version = 2;
