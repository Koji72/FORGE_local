# Contributing to Forge

Thank you for your interest in contributing to Forge! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and inclusive. We welcome contributors of all backgrounds and experience levels.

## Getting Started

### Prerequisites

- Rust 1.82+ (install via [rustup](https://rustup.rs/))
- Node.js 20+ (for frontend development)
- Ollama (optional, for AI features)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/forge/forge.git
cd forge

# Install Rust toolchain
rustup show  # This will install from rust-toolchain.toml

# Install frontend dependencies
cd apps/forge-ui
npm install
cd ../..

# Run in development mode
cargo tauri dev
```

### Running Tests

```bash
# All Rust tests
cargo test --workspace

# Specific crate
cargo test -p forge-storage

# Frontend tests
cd apps/forge-ui && npm test

# E2E tests
npm run test:e2e
```

## How to Contribute

### Reporting Bugs

1. Check if the issue already exists
2. Create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)

### Suggesting Features

1. Check existing issues and discussions
2. Open a new issue with the "feature request" template
3. Describe the use case and proposed solution

### Pull Requests

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/my-feature
   # or
   git checkout -b fix/bug-description
   ```
3. **Make your changes** following our code standards
4. **Write tests** for new functionality
5. **Run checks**:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features
   cargo test --workspace
   ```
6. **Commit** with conventional commit messages
7. **Push** and create a Pull Request

## Code Standards

### Rust

- Follow `rustfmt` defaults (run `cargo fmt`)
- Address all `clippy` warnings
- Write doc comments for public APIs
- Add tests for new functionality
- Use `thiserror` for error types
- Prefer `async/await` for I/O operations

### TypeScript/Svelte

- Follow ESLint and Prettier configs
- Use TypeScript strict mode
- Prefer composition over inheritance
- Write component documentation

### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting (no code change)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

Examples:
```
feat(agents): Add weekly review agent
fix(storage): Handle concurrent writes correctly
docs(api): Update IPC documentation
```

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation
- `refactor/description` - Code refactoring

## Project Structure

```
forge/
├── crates/           # Rust crates
│   ├── forge-domain/     # Core entities
│   ├── forge-storage/    # Database layer
│   ├── forge-indexing/   # Search
│   ├── forge-agents/     # AI agents
│   ├── forge-sync/       # E2EE sync
│   ├── forge-plugins/    # Plugin system
│   ├── forge-api-ipc/    # IPC contracts
│   └── forge-app/        # Tauri integration
├── apps/
│   └── forge-ui/         # Svelte frontend
├── tauri/
│   └── src-tauri/        # Tauri config
├── docs/                 # Documentation
├── schemas/              # JSON schemas
└── migrations/           # DB migrations
```

## Architecture Guidelines

### Backend (Rust)

1. **Domain-Driven Design**: Keep business logic in `forge-domain`
2. **Repository Pattern**: Data access through repositories in `forge-storage`
3. **Error Handling**: Use `Result<T, E>` with custom error types
4. **Async**: Use `tokio` for async operations
5. **Testing**: Unit tests in modules, integration tests in `tests/`

### Frontend (Svelte)

1. **Component-Based**: Small, focused components
2. **Stores**: Svelte stores for state management
3. **IPC**: All backend calls through typed IPC client
4. **Accessibility**: ARIA labels, keyboard navigation

### Security

1. **Never** commit secrets or credentials
2. **Validate** all inputs at boundaries
3. **Sanitize** user content before display
4. **Review** security implications of changes

## Review Process

1. All PRs require at least one approval
2. CI must pass (tests, linting, build)
3. Security-sensitive changes require security review
4. Breaking changes require discussion

## Release Process

1. Version bumps follow semver
2. Changelog updated with each release
3. Releases are tagged and signed
4. Binaries are built for all platforms

## Getting Help

- **Documentation**: Check `/docs` folder
- **Issues**: Search existing issues
- **Discussions**: Use GitHub Discussions for questions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
