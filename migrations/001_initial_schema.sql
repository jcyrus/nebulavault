-- Groups: Recursive folder structure for organizing connections
CREATE TABLE IF NOT EXISTS groups (
    id TEXT PRIMARY KEY NOT NULL,
    parent_id TEXT,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (parent_id) REFERENCES groups(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_groups_parent_id ON groups(parent_id);

-- Identities: Encrypted SSH keys and passwords
CREATE TABLE IF NOT EXISTS identities (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    encrypted_data BLOB NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Hosts: SSH connection configurations
CREATE TABLE IF NOT EXISTS hosts (
    id TEXT PRIMARY KEY NOT NULL,
    group_id TEXT,
    identity_id TEXT,
    name TEXT NOT NULL,
    hostname TEXT NOT NULL,
    port INTEGER NOT NULL DEFAULT 22,
    username TEXT NOT NULL,
    tags TEXT, -- JSON array of tags
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE SET NULL,
    FOREIGN KEY (identity_id) REFERENCES identities(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_hosts_group_id ON hosts(group_id);
CREATE INDEX IF NOT EXISTS idx_hosts_identity_id ON hosts(identity_id);
CREATE INDEX IF NOT EXISTS idx_hosts_tags ON hosts(tags);
