# Changelog

All notable changes to Nebula Vault will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Nothing yet

### Changed

- Nothing yet

### Fixed

- Nothing yet

## [0.1.0] - 2024-12-05

### Added

#### Core Infrastructure

- **Project Setup**: Initial Rust project with `iced` GUI framework and GPU acceleration via `wgpu`
- **Database Layer**: SQLite database with `sqlx` for async operations
  - Host storage schema
  - Identity storage schema
  - Database migrations system
- **Vault Encryption**: Zero-knowledge encryption system
  - Argon2id key derivation with OWASP parameters (m=19MB, t=2, p=1)
  - age encryption (XChaCha20-Poly1305) for identity data
  - Deterministic salt generation for consistent encryption keys
  - Master password unlocking on startup

#### GUI & Views

- **Code Architecture**: Refactored monolithic code into modular structure
  - `src/gui/messages.rs` - Event message types
  - `src/gui/state.rs` - Application state management
  - `src/gui/app.rs` - Update logic and message handlers
  - `src/gui/views/` - UI components split by responsibility
    - `auth.rs` - Password entry, loading, error screens
    - `main_view.rs` - Sidebar and main content area
    - `host_dialogs.rs` - Host CRUD dialogs
    - `identity_dialogs.rs` - Identity management UI
    - `terminal.rs` - SSH terminal view
- **Modern Dark Theme**: GPU-accelerated UI with glassmorphic design
- **Responsive Layout**: Sidebar + main content with scrollable lists

#### Identity Management

- **Create Identities**: Add encrypted SSH credentials
  - Password authentication
  - SSH key authentication (with optional passphrase)
- **SSH Key Input Methods**:
  - File path support (`~/.ssh/id_rsa` - auto-expands and reads file)
  - Raw key paste (single-line input)
- **Identity List**: View all stored identities with type indicators
- **Identity Encryption**: Automatic encryption before database storage
- **Identity Linking**: Associate identities with SSH hosts

#### Host Management

- **CRUD Operations**: Full create, read, update, delete for SSH hosts
- **Host Configuration**:
  - Name (display label)
  - Hostname (IP or domain)
  - Port (default 22)
  - Username
  - Linked identity (optional)
- **Inline Actions**: Edit and delete buttons directly on host items
- **Delete Confirmation**: Safety dialog before removing hosts

#### SSH Integration

- **SSH Client**: `russh` implementation for native SSH connections
- **Authentication Methods**:
  - Password authentication
  - SSH key authentication (RSA, Ed25519, etc.)
  - Passphrase-protected keys
- **Connection Flow**:
  1. Load encrypted identity from database
  2. Decrypt using vault (synchronous)
  3. Connect to SSH server (async)
  4. Transition to terminal view on success
- **Error Handling**: Detailed error messages for connection failures
- **Terminal UI**: Basic terminal view (connection successful, I/O pending)

### Changed

- **Vault Key Derivation**: Fixed to use deterministic salt instead of random
  - Ensures same password produces same encryption key across sessions
  - Enables proper decryption of stored identities
- **Error Display**: Added error message display to main UI view
  - Previously silent failures now show user-friendly messages

### Fixed

- **Critical: Vault Decryption Bug**
  - **Issue**: Random salt generation caused decryption failures
  - **Impact**: Identities encrypted in one session couldn't be decrypted in another
  - **Fix**: Implemented deterministic salt derived from password
  - **Breaking**: Existing encrypted identities cannot be recovered (require recreation)
- **Build Warnings**: Fixed all compiler warnings
  - Removed unused imports (`rand::rngs::OsRng`, `text_editor`)
  - Removed unnecessary `mut` keywords
  - Added explicit lifetimes to `Element` return types
- **Silent Connection Failures**: Added error message display to UI
- **SSH Key File Reading**: Added automatic file reading for key paths

### Security

- **Zero-Knowledge Encryption**: Master password never stored on disk
- **Encrypted Storage**: All credentials encrypted before database persistence
- **Secure Password Input**: Password fields use secure input (hidden text)
- **Memory Safety**: Rust's ownership system prevents common vulnerabilities

### Known Issues

- **SSH Data Streaming Not Implemented**: Terminal connects but doesn't stream I/O
  - Requires implementing `iced::Subscription` for async data flow
  - Workaround: Shows "streaming not implemented" message
- **Identity Editing**: Edit functionality exists but form population not implemented
- **Identity Deletion**: No UI trigger for deletion (backend exists)
- **No Form Validation**: Input fields lack validation (accept any text)
- **Single-line Key Input**: Text input strips newlines from SSH keys
  - Workaround: Use file path input instead
- **No Host Update**: `db::update_host` function not yet implemented

### Breaking Changes

- **Database**: Existing encrypted identities incompatible due to salt fix
  - **Action Required**: Delete `nebulavault.db` and recreate identities

---

## Release Notes

### Version 0.1.0 - Initial Development Release

This is the first development snapshot of Nebula Vault. The core infrastructure is in place:

- Secure vault encryption ✅
- Identity management ✅
- Host management ✅
- SSH authentication ✅
- Basic terminal UI ✅

**Not Yet Ready For**:

- Production use (still in active development)
- Real SSH workflows (data streaming not implemented)
- Long-term data storage (database schema may change)

**Next Steps**:

1. Implement SSH data streaming (Phase 2.5)
2. Complete identity editing/deletion (Phase 2.6)
3. Add terminal multiplexing (Phase 2.7)
