# 🌌 Nebula Vault

> **⚠️ WORK IN PROGRESS** - This project is under active development. Not ready for production use.

> **A beautiful, secure SSH connection manager with cinematic glassmorphism UI**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

## Overview

Nebula Vault is a **focused SSH credential manager** that securely stores your SSH credentials and launches connections in your preferred terminal application. Built with Rust and GPU-accelerated graphics, it features a stunning glassmorphism UI inspired by modern macOS applications like CleanMyMac.

### Why Nebula Vault?

**The Problem**: Terminal applications are great at being terminals, but they lack secure, convenient credential management. Nebula Vault fills this gap.

**What You Get**:

- ✅ Encrypted credential storage with zero-knowledge architecture
- ✅ Beautiful, modern glassmorphism UI
- ✅ SSH key and password management
- ✅ One-click connections to your servers
- ✅ Launches in your preferred terminal (iTerm2, Alacritty, Warp, etc.)
- ✅ Pure Rust - native performance, no Electron

**What We Don't Do**:

- ❌ Terminal emulation (use your favorite terminal instead)
- ❌ SSH session management (handled by your terminal)

## Features

### 🎨 Modern UI

- **Cinematic Glassmorphism** - Gradient backgrounds, glass effects, smooth animations
- **GPU-Accelerated** - Built with iced framework for smooth 60fps rendering
- **Dark Theme** - Easy on the eyes with vibrant accent colors
- **Native macOS Feel** - Standard window controls, native behavior

### 🔐 Security First

- **Zero-Knowledge Encryption** - Master password never stored
- **age Encryption** - Modern XChaCha20-Poly1305 encryption
- **Argon2id KDF** - OWASP-recommended key derivation (19MB, t=2, p=1)
- **Local-First** - All data stored locally, no cloud sync
- **Secure Temp Files** - SSH keys written with 0600 permissions, auto-cleanup

### 🚀 SSH Management

- **Connection Profiles** - Save your frequently-used servers
- **Identity Management** - Store SSH keys and passwords securely
- **Terminal Launcher** - Opens connections in your preferred terminal
- **Quick Connect** - One click to launch SSH session

## Supported Terminals

### macOS

- **iTerm2** (recommended)
- **Terminal.app** (default)
- **Warp**
- **Alacritty**
- **Kitty**

### Linux/Windows

- **Alacritty**
- **Kitty**
- **Custom command**

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/jcyrus/nebulavault.git
cd nebulavault

# Build release version
cargo build --release

# Run
./target/release/nebulavault
```

### Requirements

- Rust 1.75 or later
- SQLite 3
- A supported terminal application
- macOS 10.15+ (for native window features)

## Quick Start

### For Testing/Development

If you're testing the app with the included database, use the default password:

- **Default Password**: `test123`

> **Note**: This is for development/testing only. In production, you'll create your own master password on first launch.

### First-Time Setup

1. **First Launch** - Create a master password (this encrypts all your credentials)
2. **Add Identity** - Store your SSH key or password
   - Click "🔑 Manage Identities"
   - Add new identity with name, type (key/password), and credentials
3. **Add Host** - Create a connection profile
   - Click "+" in the sidebar
   - Enter name, hostname, port, username
   - Link to an identity (optional)
4. **Connect** - Click any host to launch terminal with SSH connection

## Tech Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[iced](https://github.com/iced-rs/iced)** - GPU-accelerated cross-platform GUI (v0.13)
- **[sqlx](https://github.com/launchbadge/sqlx)** - Async SQL with compile-time verification
- **[age](https://github.com/str4d/rage)** - Modern encryption library
- **[russh](https://github.com/warp-tech/russh)** - Pure Rust SSH implementation

## Architecture

```
┌─────────────────────────────────────┐
│         Nebula Vault GUI            │
│  (iced + GPU-accelerated graphics)  │
└──────────────┬──────────────────────┘
               │
       ┌───────┴────────┐
       │                │
   ┌───▼────┐      ┌────▼────┐
   │ Vault  │      │Terminal │
   │(age)   │      │Launcher │
   └───┬────┘      └────┬────┘
       │                │
   ┌───▼────┐      ┌────▼────────┐
   │SQLite  │      │iTerm2/Warp/ │
   │Database│      │Alacritty/etc│
   └────────┘      └─────────────┘
```

## Security Model

### Zero-Knowledge Architecture

1. Master password entered by user
2. Argon2id derives encryption key (never stored)
3. Credentials encrypted with age before storage
4. Decryption only in memory when needed
5. Temporary SSH key files cleaned up immediately

### What We Store

- **Encrypted**: SSH keys, passwords, passphrases
- **Plaintext**: Host names, usernames, ports (non-sensitive metadata)

### What We Don't Store

- Master password (never touches disk)
- Decrypted credentials (memory only)
- Usage analytics or telemetry

## Current Status

**✅ Complete**:

- Vault encryption system
- Identity management (SSH keys & passwords)
- Host profile management
- Terminal launcher (macOS)
- Settings UI for terminal preference
- Glassmorphism UI with gradients and animations
- Native window controls

**🚧 In Progress**:

- Cross-platform terminal support (Linux/Windows)
- Advanced UI polish

**📋 Planned**:

- Host grouping/folders
- Connection history
- Import/export profiles
- Global hotkey support
- Optional cloud sync (encrypted)

## Why Not Build a Terminal?

We initially considered building a full terminal emulator, but realized:

1. **Terminals are hard** - iTerm2 has 15+ years of development
2. **Users have preferences** - Everyone loves their terminal setup
3. **Focus matters** - Better to excel at credential management
4. **Faster shipping** - Deliver value sooner by using existing terminals

Nebula Vault focuses on what terminals **don't have**: secure, beautiful credential management.

## Development

### Project Structure

```
nebulavault/
├── src/
│   ├── db/              # Database layer (SQLite + sqlx)
│   ├── vault/           # Encryption (age + argon2)
│   ├── models/          # Data models
│   ├── terminal_launcher/ # Terminal integration
│   └── gui/             # UI layer
│       ├── app.rs       # Application logic
│       ├── state.rs     # State management
│       ├── messages.rs  # Event messages
│       ├── views/       # UI components
│       └── widgets/     # Custom widgets
├── assets/              # Icons and resources
└── Cargo.toml
```

### Building for Development

```bash
# Debug build (faster compile)
cargo build

# Run with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Check without building
cargo check
```

## Contributing

This is a personal project in active development. The codebase is evolving rapidly.

If you'd like to contribute:

1. Open an issue to discuss your idea
2. Fork the repository
3. Create a feature branch
4. Submit a pull request

## License

MIT License - See [LICENSE](./LICENSE) for details

## Acknowledgments

Built with excellent Rust crates:

- [iced](https://github.com/iced-rs/iced) - Beautiful GPU-accelerated GUI
- [sqlx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [age](https://github.com/str4d/rage) - Modern encryption
- [russh](https://github.com/warp-tech/russh) - Pure Rust SSH

Inspired by:

- CleanMyMac's beautiful glassmorphism UI
- 1Password's focus on security and UX
- iTerm2's terminal excellence

---

**⚠️ Note**: This is experimental software under active development. Use at your own risk. Always keep backups of important SSH keys.
