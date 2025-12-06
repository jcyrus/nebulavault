use anyhow::{Context, Result};
use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;

use crate::models::{Group, Host, Identity};

/// Initialize the SQLite database and run migrations
pub async fn init_db(db_path: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path))
        .await
        .context("Failed to connect to database")?;

    // Run migrations - use macro which embeds migrations at compile time
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}

// ============================================================================
// Groups
// ============================================================================

/// Create a new group
pub async fn create_group(
    pool: &SqlitePool,
    parent_id: Option<String>,
    name: String,
) -> Result<Group> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO groups (id, parent_id, name, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&parent_id)
    .bind(&name)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .context("Failed to create group")?;

    Ok(Group {
        id,
        parent_id,
        name,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Get all groups
pub async fn get_all_groups(pool: &SqlitePool) -> Result<Vec<Group>> {
    let groups = sqlx::query_as::<_, Group>("SELECT * FROM groups ORDER BY name")
        .fetch_all(pool)
        .await
        .context("Failed to fetch groups")?;

    Ok(groups)
}

/// Delete a group
pub async fn delete_group(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM groups WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .context("Failed to delete group")?;

    Ok(())
}

// ============================================================================
// Identities
// ============================================================================

/// Create a new identity
pub async fn create_identity(
    pool: &SqlitePool,
    name: String,
    encrypted_data: Vec<u8>,
) -> Result<Identity> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO identities (id, name, encrypted_data, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&name)
    .bind(&encrypted_data)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .context("Failed to create identity")?;

    Ok(Identity {
        id,
        name,
        encrypted_data,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Get a single identity by ID
pub async fn get_identity(pool: &SqlitePool, id: &str) -> Result<Option<Identity>> {
    let identity = sqlx::query_as::<_, Identity>(
        "SELECT id, name, encrypted_data, created_at, updated_at FROM identities WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch identity")?;

    Ok(identity)
}

/// Get all identities
pub async fn get_all_identities(pool: &SqlitePool) -> Result<Vec<Identity>> {
    let identities = sqlx::query_as::<_, Identity>("SELECT * FROM identities ORDER BY name")
        .fetch_all(pool)
        .await
        .context("Failed to fetch identities")?;

    Ok(identities)
}

/// Get identity by ID
pub async fn get_identity_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Identity>> {
    let identity = sqlx::query_as::<_, Identity>("SELECT * FROM identities WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .context("Failed to fetch identity")?;

    Ok(identity)
}

/// Delete an identity
pub async fn delete_identity(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM identities WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .context("Failed to delete identity")?;

    Ok(())
}

// ============================================================================
// Hosts
// ============================================================================

/// Create a new host
pub async fn create_host(
    pool: &SqlitePool,
    group_id: Option<String>,
    identity_id: Option<String>,
    name: String,
    hostname: String,
    port: i64,
    username: String,
    tags: Option<String>,
) -> Result<Host> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO hosts (id, group_id, identity_id, name, hostname, port, username, tags, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&group_id)
    .bind(&identity_id)
    .bind(&name)
    .bind(&hostname)
    .bind(port)
    .bind(&username)
    .bind(&tags)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .context("Failed to create host")?;

    Ok(Host {
        id,
        group_id,
        identity_id,
        name,
        hostname,
        port,
        username,
        tags,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Update an existing host
pub async fn update_host(
    pool: &SqlitePool,
    id: &str,
    name: String,
    hostname: String,
    port: i64,
    username: String,
    identity_id: Option<String>,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE hosts SET name = ?, hostname = ?, port = ?, username = ?, identity_id = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&name)
    .bind(&hostname)
    .bind(port)
    .bind(&username)
    .bind(&identity_id)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await
    .context("Failed to update host")?;

    Ok(())
}

/// Get all hosts
pub async fn get_all_hosts(pool: &SqlitePool) -> Result<Vec<Host>> {
    let hosts = sqlx::query_as::<_, Host>("SELECT * FROM hosts ORDER BY name")
        .fetch_all(pool)
        .await
        .context("Failed to fetch hosts")?;

    Ok(hosts)
}

/// Get host by ID
pub async fn get_host_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Host>> {
    let host = sqlx::query_as::<_, Host>("SELECT * FROM hosts WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .context("Failed to fetch host")?;

    Ok(host)
}

/// Delete a host
pub async fn delete_host(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM hosts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .context("Failed to delete host")?;

    Ok(())
}

/// Check if database is empty (no hosts)
pub async fn is_database_empty(pool: &SqlitePool) -> Result<bool> {
    let count: i64 = sqlx::query("SELECT COUNT(*) as count FROM hosts")
        .fetch_one(pool)
        .await
        .context("Failed to count hosts")?
        .try_get("count")
        .context("Failed to get count")?;

    Ok(count == 0)
}
