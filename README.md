# Forge

<p align="center">
  <strong>Local-first productivity app with AI agents and optional E2EE sync</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## Overview

Forge is a **local-first** productivity application that combines task management, note-taking, and goal tracking with AI-powered agents. All your data stays on your device by default, with optional end-to-end encrypted sync between devices.

Built with **Rust** and **Tauri** for native performance, and **Svelte** for a modern, responsive UI.

## Features

### Core Functionality
- **📋 Task Management** - GTD-style inbox, next actions, waiting, someday/maybe
- **📝 Note Taking** - Markdown notes with types (note, meeting, journal, reference)
- **🎯 Goal Tracking** - Hierarchical goals with horizons (short/medium/long/vision)
- **🔗 Entity Linking** - Connect tasks, notes, and goals in a knowledge graph

### AI Agents
- **Prioritize Inbox** - AI-powered task prioritization with explanations
- **Weekly Review** - Automated weekly progress summaries
- **Extract Tasks** - Pull actionable items from meeting notes
- **Summarize Notes** - Generate concise summaries with key points
- **Plan Day** - Intelligent daily scheduling based on energy and priorities

### Search & Discovery
- **Full-Text Search** - Fast FTS5-powered search across all content
- **Semantic Search** - Find related content using embeddings (via Ollama)
- **Hybrid Search** - Combined BM25 + vector similarity ranking

### Privacy & Security
- **🔐 Local-First** - All data stored locally, works offline
- **🔒 Encrypted Storage** - SQLCipher with AES-256 encryption
- **🛡️ E2EE Sync** - Optional sync with XChaCha20-Poly1305 encryption
- **🚫 No Telemetry** - Zero data collection by default

### Extensibility
- **Plugin System** - Extend functionality with sandboxed plugins
- **Permission-Based** - Granular capability controls for plugins
- **Signed Plugins** - Ed25519 signature verification

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Tauri Shell                          │
├─────────────────────────────────────────────────────────────┤
│                     Svelte 5 + SvelteKit                    │
├─────────────────────────────────────────────────────────────┤
│                        IPC Layer                            │
├──────────┬──────────┬──────────┬──────────┬────────────────┤
│  Domain  │ Storage  │ Indexing │  Agents  │     Sync       │
│          │(SQLCipher│ (FTS5 +  │ (Ollama) │  (yrs CRDT +   │
│          │   WAL)   │  HNSW)   │          │    E2EE)       │
├──────────┴──────────┴──────────┴──────────┴────────────────┤
│                       Plugins                               │
└─────────────────────────────────────────────────────────────┘
```

### Crate Structure

| Crate | Description |
|-------|-------------|
| `forge-domain` | Core entities, validation rules, business logic |
| `forge-storage` | SQLite/SQLCipher database layer, migrations |
| `forge-indexing` | FTS5 search, embeddings, HNSW vector index |
| `forge-agents` | LLM orchestration, tools, schema validation |
| `forge-sync` | CRDT sync, E2EE crypto, WebSocket relay client |
| `forge-plugins` | Plugin manifest, permissions, sandbox runtime |
| `forge-api-ipc` | IPC contracts, DTOs, error definitions |
| `forge-app` | Tauri integration, command handlers, state |

## Tech Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust 1.82+ |
| Desktop Framework | Tauri 2.9+ |
| Frontend | Svelte 5, SvelteKit 2.x |
| Database | SQLite 3 + SQLCipher (AES-256) |
| Full-Text Search | FTS5 |
| Vector Search | HNSW (embedded) |
| LLM Runtime | Ollama (local) |
| CRDT | yrs (Yjs Rust port) |
| Crypto | libsodium (XChaCha20-Poly1305, Ed25519, Argon2id) |

## Getting Started

### Prerequisites

- **Rust** 1.82+ ([install](https://rustup.rs/))
- **Node.js** 20+ ([install](https://nodejs.org/))
- **Ollama** (optional, for AI features) ([install](https://ollama.ai/))

### System Dependencies

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get install -y \
  libsqlite3-dev libssl-dev pkg-config \
  libgtk-3-dev libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev librsvg2-dev
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Installation

```bash
# Clone the repository
git clone https://github.com/forge/forge.git
cd forge

# Install frontend dependencies
cd apps/forge-ui
npm install
cd ../..

# Build the application
cargo build --release

# Run in development mode
cargo tauri dev
```

### Running with Ollama

For AI features, start Ollama with a compatible model:

```bash
# Install models
ollama pull llama3
ollama pull nomic-embed-text

# Start Ollama (usually runs automatically)
ollama serve
```

## Project Structure

```
forge/
├── crates/                    # Rust crates
│   ├── forge-domain/          # Domain entities
│   ├── forge-storage/         # Database layer
│   ├── forge-indexing/        # Search indexing
│   ├── forge-agents/          # AI agents
│   ├── forge-sync/            # E2EE sync
│   ├── forge-plugins/         # Plugin system
│   ├── forge-api-ipc/         # IPC contracts
│   └── forge-app/             # App integration
├── apps/
│   └── forge-ui/              # Svelte frontend
├── tauri/
│   └── src-tauri/             # Tauri configuration
├── docs/                      # Documentation
│   ├── spec-tech.md           # Technical specification
│   ├── api-ipc.md             # API reference
│   ├── sync-protocol.md       # Sync protocol
│   ├── threat-model.md        # Security analysis
│   └── plugin-dev.md          # Plugin guide
├── schemas/
│   └── agents/                # Agent JSON schemas
├── migrations/                # Database migrations
└── .github/workflows/         # CI/CD pipelines
```

## Documentation

| Document | Description |
|----------|-------------|
| [Technical Specification](docs/spec-tech.md) | Complete system design and requirements |
| [API Reference](docs/api-ipc.md) | IPC commands, DTOs, and events |
| [Sync Protocol](docs/sync-protocol.md) | E2EE sync wire protocol |
| [Threat Model](docs/threat-model.md) | Security analysis and mitigations |
| [Plugin Development](docs/plugin-dev.md) | Guide to building plugins |

## Development

### Running Tests

```bash
# Run all Rust tests
cargo test --workspace

# Run with coverage
cargo tarpaulin --workspace

# Run frontend tests
cd apps/forge-ui && npm test
```

### Linting & Formatting

```bash
# Rust
cargo fmt --all
cargo clippy --all-targets --all-features

# Frontend
cd apps/forge-ui
npm run lint
npm run format
```

### Database Migrations

Migrations are in `migrations/` and run automatically on startup.

```bash
# View current schema
sqlite3 forge.db ".schema"
```

## Configuration

Default settings are stored in the database. Key configurations:

| Setting | Default | Description |
|---------|---------|-------------|
| `llm.provider` | `ollama` | LLM provider |
| `llm.ollama.base_url` | `http://127.0.0.1:11434` | Ollama endpoint |
| `llm.model.chat` | `llama3` | Chat model |
| `llm.model.embed` | `nomic-embed-text` | Embedding model |
| `agents.timeout_ms` | `30000` | Agent timeout |
| `sync.enabled` | `false` | Enable sync |
| `backups.keep` | `5` | Backup retention |

## Roadmap

| Phase | Focus |
|-------|-------|
| Sprint 1-2 | Tauri + IPC + Storage + CRUD |
| Sprint 3 | FTS + Export/Import + Backups |
| Sprint 4 | Agents + Schema validation |
| Sprint 5 | Embeddings + Semantic search |
| Sprint 6 | Sync (CRDT + E2EE + Relay) |
| Sprint 7 | Plugins v0 |
| Sprint 8 | Hardening + QA + Release |

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Standards

- Rust: Follow `rustfmt` and `clippy` defaults
- TypeScript/Svelte: ESLint + Prettier
- Commits: Conventional Commits format
- Tests: Required for new features

## Security

- **Reporting Vulnerabilities**: Please report security issues privately via email
- **Security Model**: See [Threat Model](docs/threat-model.md)
- **Encryption**: SQLCipher (AES-256) + XChaCha20-Poly1305 for sync

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  Made with ❤️ by the Forge Team
</p>
