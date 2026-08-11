CREATE TABLE IF NOT EXISTS conversations (
    id BLOB PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id BLOB PRIMARY KEY NOT NULL,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    screenshot_path TEXT,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS transcripts (
    id BLOB PRIMARY KEY NOT NULL,
    conversation_id TEXT NOT NULL,
    source TEXT NOT NULL,
    text TEXT NOT NULL,
    confidence REAL NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS system_prompts (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    prompt TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS llm_provider_settings (
    id TEXT PRIMARY KEY NOT NULL,
    settings TEXT NOT NULL
);

INSERT OR IGNORE INTO llm_provider_settings (id, settings) VALUES ('fake', '{"type":"Fake"}');

CREATE TABLE IF NOT EXISTS stt_providers_settings (
    id TEXT PRIMARY KEY NOT NULL,
    settings TEXT NOT NULL
);

INSERT OR IGNORE INTO stt_providers_settings (id, settings) VALUES ('fake', '{"type":"Fake"}');

CREATE TABLE IF NOT EXISTS shortcut_overrides (
    id TEXT PRIMARY KEY NOT NULL,
    key_override TEXT,
    enabled INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_transcripts_conversation ON transcripts(conversation_id);
