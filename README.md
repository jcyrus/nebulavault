# 🌌 Nebula Vault

> **⚠️ WORK IN PROGRESS** - This project is under active development. Not ready for production use.

A modern, secure SSH connection manager built with Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

## Overview

Nebula Vault is a desktop application for managing SSH connections with a focus on security and performance. Built entirely in Rust with a GPU-accelerated interface.

## Features

- 🔐 **Secure credential storage** with client-side encryption
- 🔑 **SSH key management** - Support for password and key-based authentication
- 🖥️ **Modern UI** - GPU-accelerated interface using iced
- 💾 **Local-first** - All data stored locally with SQLite
- 🦀 **Pure Rust** - Native performance, no Electron

## Tech Stack

- **Rust** - Systems programming language
- **iced** - Cross-platform GUI framework
- **russh** - SSH client implementation
- **sqlx** - Async SQL database
- **age** - Modern encryption library

## Building

```bash
# Clone the repository
git clone https://github.com/jcyrus/nebulavault.git
cd nebulavault

# Build and run
cargo build --release
./target/release/nebulavault
```

### Requirements

- Rust 1.75 or later
- SQLite 3

## Current Status

This project is in early development. Core features are being implemented:

- ✅ Basic UI framework
- ✅ Database layer
- ✅ Encryption system
- ✅ SSH authentication
- 🚧 Terminal integration
- 📋 Advanced features (planned)

## Security

- Master password required on startup (never stored)
- Client-side encryption for all credentials
- Zero-knowledge architecture
- No telemetry or data collection

## Roadmap

- [ ] Complete terminal integration
- [ ] SSH session management
- [ ] SFTP support
- [ ] Port forwarding
- [ ] Cloud sync (optional, encrypted)

## Contributing

This is a personal project in active development. The codebase is changing rapidly and the API is not stable.

If you're interested in contributing, please open an issue first to discuss your ideas.

## License

MIT License - See [LICENSE](./LICENSE) for details

## Acknowledgments

Built with excellent Rust crates:

- [iced](https://github.com/iced-rs/iced) - GUI framework
- [russh](https://github.com/warp-tech/russh) - SSH implementation
- [sqlx](https://github.com/launchbadge/sqlx) - Database toolkit
- [age](https://github.com/str4d/rage) - Encryption library

---

**Note**: This is experimental software. Use at your own risk.
