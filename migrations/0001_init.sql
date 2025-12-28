-- =============================================================================
-- Forge v1.0 - Initial Database Schema
-- Migration: 0001_init.sql
-- =============================================================================
-- This migration creates all base tables for Forge's local-first vault.
-- Database: SQLite 3.x with SQLCipher (AES-256 encryption)
-- =============================================================================

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- =============================================================================
-- CORE ENTITIES
-- =============================================================================

-- -----------------------------------------------------------------------------
-- Tasks
-- -----------------------------------------------------------------------------
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,                          -- UUIDv7
    title TEXT NOT NULL,
    description_md TEXT,                          -- Markdown content
    status TEXT NOT NULL DEFAULT 'inbox',         -- inbox|next|waiting|someday|done|archived
    priority INTEGER,                             -- 1 (highest) to 5 (lowest), nullable
    priority_score REAL,                          -- AI-computed score 0-100
    due_at INTEGER,                               -- Epoch ms UTC, nullable
    scheduled_at INTEGER,                         -- Epoch ms UTC, nullable
    completed_at INTEGER,                         -- Epoch ms UTC, nullable
    parent_task_id TEXT REFERENCES tasks(id)
        ON DELETE SET NULL,                       -- For subtasks
    goal_id TEXT REFERENCES goals(id)
        ON DELETE SET NULL,                       -- Associated goal
    context_tags TEXT,                            -- JSON array of strings
    energy_level TEXT,                            -- low|medium|high
    time_estimate_min INTEGER,                    -- Estimated minutes
    recurrence_rule TEXT,                         -- iCal RRULE string
    created_at INTEGER NOT NULL,                  -- Epoch ms UTC
    updated_at INTEGER NOT NULL,                  -- Epoch ms UTC
    deleted_at INTEGER,                           -- Soft delete, epoch ms UTC

    -- Constraints
    CHECK (status IN ('inbox', 'next', 'waiting', 'someday', 'done', 'archived')),
    CHECK (priority IS NULL OR (priority >= 1 AND priority <= 5)),
    CHECK (priority_score IS NULL OR (priority_score >= 0 AND priority_score <= 100)),
    CHECK (energy_level IS NULL OR energy_level IN ('low', 'medium', 'high'))
);

CREATE INDEX idx_tasks_status ON tasks(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_due_at ON tasks(due_at) WHERE deleted_at IS NULL AND due_at IS NOT NULL;
CREATE INDEX idx_tasks_goal_id ON tasks(goal_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_parent ON tasks(parent_task_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_updated_at ON tasks(updated_at);

-- -----------------------------------------------------------------------------
-- Notes
-- -----------------------------------------------------------------------------
CREATE TABLE notes (
    id TEXT PRIMARY KEY,                          -- UUIDv7
    title TEXT NOT NULL,
    content_md TEXT NOT NULL,                     -- Markdown content
    note_type TEXT NOT NULL DEFAULT 'note',       -- note|meeting|journal|reference
    tags TEXT,                                    -- JSON array of strings
    pinned INTEGER NOT NULL DEFAULT 0,            -- Boolean
    created_at INTEGER NOT NULL,                  -- Epoch ms UTC
    updated_at INTEGER NOT NULL,                  -- Epoch ms UTC
    deleted_at INTEGER,                           -- Soft delete

    CHECK (note_type IN ('note', 'meeting', 'journal', 'reference')),
    CHECK (pinned IN (0, 1))
);

CREATE INDEX idx_notes_type ON notes(note_type) WHERE deleted_at IS NULL;
CREATE INDEX idx_notes_pinned ON notes(pinned) WHERE deleted_at IS NULL AND pinned = 1;
CREATE INDEX idx_notes_updated_at ON notes(updated_at);

-- -----------------------------------------------------------------------------
-- Goals
-- -----------------------------------------------------------------------------
CREATE TABLE goals (
    id TEXT PRIMARY KEY,                          -- UUIDv7
    title TEXT NOT NULL,
    description_md TEXT,
    status TEXT NOT NULL DEFAULT 'active',        -- active|completed|paused|archived
    horizon TEXT NOT NULL DEFAULT 'medium',       -- short|medium|long|vision
    target_date INTEGER,                          -- Epoch ms UTC
    progress_percent INTEGER DEFAULT 0,           -- 0-100
    parent_goal_id TEXT REFERENCES goals(id)
        ON DELETE SET NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER,

    CHECK (status IN ('active', 'completed', 'paused', 'archived')),
    CHECK (horizon IN ('short', 'medium', 'long', 'vision')),
    CHECK (progress_percent >= 0 AND progress_percent <= 100)
);

CREATE INDEX idx_goals_status ON goals(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_goals_horizon ON goals(horizon) WHERE deleted_at IS NULL;

-- -----------------------------------------------------------------------------
-- Links (Entity Graph)
-- -----------------------------------------------------------------------------
CREATE TABLE links (
    id TEXT PRIMARY KEY,                          -- UUIDv7
    from_type TEXT NOT NULL,                      -- task|note|goal
    from_id TEXT NOT NULL,
    to_type TEXT NOT NULL,                        -- task|note|goal
    to_id TEXT NOT NULL,
    label TEXT,                                   -- Optional relationship label
    created_at INTEGER NOT NULL,

    CHECK (from_type IN ('task', 'note', 'goal')),
    CHECK (to_type IN ('task', 'note', 'goal')),
    UNIQUE(from_type, from_id, to_type, to_id)
);

CREATE INDEX idx_links_from ON links(from_type, from_id);
CREATE INDEX idx_links_to ON links(to_type, to_id);

-- =============================================================================
-- AGENT SYSTEM
-- =============================================================================

-- -----------------------------------------------------------------------------
-- Agent Runs
-- -----------------------------------------------------------------------------
CREATE TABLE agent_runs (
    id TEXT PRIMARY KEY,                          -- UUIDv7
    agent_type TEXT NOT NULL,                     -- e.g., prioritize_inbox, weekly_review
    status TEXT NOT NULL DEFAULT 'pending',       -- pending|running|completed|failed|cancelled
    input_json TEXT NOT NULL,                     -- Input context as JSON
    output_json TEXT,                             -- Output result as JSON
    error_json TEXT,                              -- Error details if failed
    tokens_used INTEGER,                          -- Total tokens consumed
    duration_ms INTEGER,                          -- Execution duration
    started_at INTEGER,                           -- Epoch ms UTC
    completed_at INTEGER,                         -- Epoch ms UTC
    created_at INTEGER NOT NULL,

    CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled'))
);

CREATE INDEX idx_agent_runs_type ON agent_runs(agent_type);
CREATE INDEX idx_agent_runs_status ON agent_runs(status);
CREATE INDEX idx_agent_runs_created ON agent_runs(created_at DESC);

-- =============================================================================
-- INDEXING & SEARCH
-- =============================================================================

-- -----------------------------------------------------------------------------
-- Embeddings
-- -----------------------------------------------------------------------------
CREATE TABLE embeddings (
    id TEXT PRIMARY KEY,                          -- UUIDv7
    entity_type TEXT NOT NULL,                    -- task|note|goal
    entity_id TEXT NOT NULL,
    model_id TEXT NOT NULL,                       -- e.g., nomic-embed-text
    vector BLOB NOT NULL,                         -- Float32 array as bytes
    dim INTEGER NOT NULL,                         -- Vector dimension
    content_hash TEXT NOT NULL,                   -- SHA256 of source content
    created_at INTEGER NOT NULL,

    CHECK (entity_type IN ('task', 'note', 'goal')),
    UNIQUE(entity_type, entity_id, model_id)
);

CREATE INDEX idx_embeddings_entity ON embeddings(entity_type, entity_id);
CREATE INDEX idx_embeddings_model ON embeddings(model_id);

-- -----------------------------------------------------------------------------
-- FTS5 Virtual Tables for Full-Text Search
-- -----------------------------------------------------------------------------

-- Notes FTS
CREATE VIRTUAL TABLE fts_notes USING fts5(
    title,
    content_md,
    content='notes',
    content_rowid='rowid'
);

-- Tasks FTS
CREATE VIRTUAL TABLE fts_tasks USING fts5(
    title,
    description_md,
    content='tasks',
    content_rowid='rowid'
);

-- Goals FTS
CREATE VIRTUAL TABLE fts_goals USING fts5(
    title,
    description_md,
    content='goals',
    content_rowid='rowid'
);

-- =============================================================================
-- FTS TRIGGERS
-- =============================================================================

-- -----------------------------------------------------------------------------
-- Notes FTS Triggers
-- -----------------------------------------------------------------------------
CREATE TRIGGER notes_ai AFTER INSERT ON notes BEGIN
    INSERT INTO fts_notes(rowid, title, content_md)
    VALUES (NEW.rowid, NEW.title, NEW.content_md);
END;

CREATE TRIGGER notes_ad AFTER DELETE ON notes BEGIN
    INSERT INTO fts_notes(fts_notes, rowid, title, content_md)
    VALUES ('delete', OLD.rowid, OLD.title, OLD.content_md);
END;

CREATE TRIGGER notes_au AFTER UPDATE ON notes BEGIN
    INSERT INTO fts_notes(fts_notes, rowid, title, content_md)
    VALUES ('delete', OLD.rowid, OLD.title, OLD.content_md);
    INSERT INTO fts_notes(rowid, title, content_md)
    VALUES (NEW.rowid, NEW.title, NEW.content_md);
END;

-- -----------------------------------------------------------------------------
-- Tasks FTS Triggers
-- -----------------------------------------------------------------------------
CREATE TRIGGER tasks_ai AFTER INSERT ON tasks BEGIN
    INSERT INTO fts_tasks(rowid, title, description_md)
    VALUES (NEW.rowid, NEW.title, NEW.description_md);
END;

CREATE TRIGGER tasks_ad AFTER DELETE ON tasks BEGIN
    INSERT INTO fts_tasks(fts_tasks, rowid, title, description_md)
    VALUES ('delete', OLD.rowid, OLD.title, OLD.description_md);
END;

CREATE TRIGGER tasks_au AFTER UPDATE ON tasks BEGIN
    INSERT INTO fts_tasks(fts_tasks, rowid, title, description_md)
    VALUES ('delete', OLD.rowid, OLD.title, OLD.description_md);
    INSERT INTO fts_tasks(rowid, title, description_md)
    VALUES (NEW.rowid, NEW.title, NEW.description_md);
END;

-- -----------------------------------------------------------------------------
-- Goals FTS Triggers
-- -----------------------------------------------------------------------------
CREATE TRIGGER goals_ai AFTER INSERT ON goals BEGIN
    INSERT INTO fts_goals(rowid, title, description_md)
    VALUES (NEW.rowid, NEW.title, NEW.description_md);
END;

CREATE TRIGGER goals_ad AFTER DELETE ON goals BEGIN
    INSERT INTO fts_goals(fts_goals, rowid, title, description_md)
    VALUES ('delete', OLD.rowid, OLD.title, OLD.description_md);
END;

CREATE TRIGGER goals_au AFTER UPDATE ON goals BEGIN
    INSERT INTO fts_goals(fts_goals, rowid, title, description_md)
    VALUES ('delete', OLD.rowid, OLD.title, OLD.description_md);
    INSERT INTO fts_goals(rowid, title, description_md)
    VALUES (NEW.rowid, NEW.title, NEW.description_md);
END;

-- =============================================================================
-- SYNC SYSTEM
-- =============================================================================

-- -----------------------------------------------------------------------------
-- Sync Devices
-- -----------------------------------------------------------------------------
CREATE TABLE sync_devices (
    device_id TEXT PRIMARY KEY,                   -- UUIDv7
    device_name TEXT,
    public_sign_key BLOB NOT NULL,                -- Ed25519 public key
    public_encrypt_key BLOB,                      -- X25519 public key (optional)
    added_at INTEGER NOT NULL,
    last_seen_at INTEGER,
    is_current INTEGER NOT NULL DEFAULT 0,        -- Boolean: is this device?

    CHECK (is_current IN (0, 1))
);

-- -----------------------------------------------------------------------------
-- Sync State
-- -----------------------------------------------------------------------------
CREATE TABLE sync_state (
    key TEXT PRIMARY KEY,
    value_json TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Default sync state values
INSERT INTO sync_state (key, value_json, updated_at) VALUES
    ('sync_enabled', 'false', 0),
    ('relay_url', '""', 0),
    ('vault_id', '""', 0),
    ('last_push_cursor', '0', 0),
    ('last_pull_cursor', '0', 0),
    ('key_version', '1', 0);

-- -----------------------------------------------------------------------------
-- Sync Pending Changes (Outbox)
-- -----------------------------------------------------------------------------
CREATE TABLE sync_outbox (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    operation TEXT NOT NULL,                      -- insert|update|delete
    payload_json TEXT NOT NULL,                   -- Encrypted delta
    created_at INTEGER NOT NULL,
    sent_at INTEGER,                              -- NULL until sent

    CHECK (operation IN ('insert', 'update', 'delete'))
);

CREATE INDEX idx_sync_outbox_pending ON sync_outbox(sent_at) WHERE sent_at IS NULL;

-- =============================================================================
-- PLUGINS
-- =============================================================================

-- -----------------------------------------------------------------------------
-- Installed Plugins
-- -----------------------------------------------------------------------------
CREATE TABLE plugins (
    plugin_id TEXT PRIMARY KEY,                   -- e.g., com.forge.example
    version TEXT NOT NULL,                        -- Semver
    enabled INTEGER NOT NULL DEFAULT 0,           -- Boolean
    manifest_json TEXT NOT NULL,                  -- Full manifest
    permissions_json TEXT NOT NULL,               -- Approved permissions
    signature_verified INTEGER NOT NULL DEFAULT 0,-- Boolean
    installed_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    CHECK (enabled IN (0, 1)),
    CHECK (signature_verified IN (0, 1))
);

-- -----------------------------------------------------------------------------
-- Plugin State (Key-Value per plugin)
-- -----------------------------------------------------------------------------
CREATE TABLE plugin_state (
    plugin_id TEXT NOT NULL REFERENCES plugins(plugin_id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value_json TEXT NOT NULL,
    updated_at INTEGER NOT NULL,

    PRIMARY KEY (plugin_id, key)
);

-- =============================================================================
-- SETTINGS & CONFIGURATION
-- =============================================================================

-- -----------------------------------------------------------------------------
-- User Settings
-- -----------------------------------------------------------------------------
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value_json TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Default settings
INSERT INTO settings (key, value_json, updated_at) VALUES
    ('llm.provider', '"ollama"', 0),
    ('llm.ollama.base_url', '"http://127.0.0.1:11434"', 0),
    ('llm.model.chat', '"llama3"', 0),
    ('llm.model.embed', '"nomic-embed-text"', 0),
    ('agents.timeout_ms', '30000', 0),
    ('agents.max_concurrent', '1', 0),
    ('indexing.embed_on_idle', 'true', 0),
    ('indexing.batch_size', '10', 0),
    ('sync.enabled', 'false', 0),
    ('sync.relay_url', '""', 0),
    ('security.vault_encryption', 'true', 0),
    ('backups.enabled', 'true', 0),
    ('backups.keep', '5', 0),
    ('backups.interval_hours', '24', 0),
    ('ui.theme', '"system"', 0),
    ('ui.language', '"en"', 0);

-- =============================================================================
-- BACKUPS & RECOVERY
-- =============================================================================

-- -----------------------------------------------------------------------------
-- Backup History
-- -----------------------------------------------------------------------------
CREATE TABLE backup_history (
    id TEXT PRIMARY KEY,                          -- UUIDv7
    file_path TEXT NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    checksum TEXT NOT NULL,                       -- SHA256
    created_at INTEGER NOT NULL,
    expires_at INTEGER,                           -- For auto-cleanup
    is_manual INTEGER NOT NULL DEFAULT 0,         -- Manual vs automatic

    CHECK (is_manual IN (0, 1))
);

CREATE INDEX idx_backup_history_created ON backup_history(created_at DESC);

-- =============================================================================
-- SCHEMA VERSION
-- =============================================================================

CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL,
    checksum TEXT NOT NULL                        -- Migration file checksum
);

INSERT INTO schema_version (version, applied_at, checksum)
VALUES (1, strftime('%s', 'now') * 1000, 'MIGRATION_CHECKSUM_PLACEHOLDER');

-- =============================================================================
-- END OF MIGRATION 0001
-- =============================================================================
