# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in Forge, please report it responsibly:

1. **DO NOT** open a public issue
2. Email security details to: [security@forge.local] (placeholder)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will acknowledge receipt within 48 hours and provide a detailed response within 7 days.

## Security Model

Forge is designed with security as a core principle:

### Data Protection

| Layer | Protection |
|-------|------------|
| Database | SQLCipher (AES-256-CBC) |
| Key Derivation | Argon2id (64MB, 3 iterations) |
| Sync Encryption | XChaCha20-Poly1305 |
| Signatures | Ed25519 |

### Threat Model

See [docs/threat-model.md](docs/threat-model.md) for detailed analysis.

Key mitigations:
- **Data at rest**: Encrypted with user passphrase
- **Data in transit**: E2EE for all sync traffic
- **Plugin isolation**: Capability-based permissions
- **XSS prevention**: Strict CSP, input sanitization

### What We Protect Against

- Unauthorized access to vault data
- Man-in-the-middle attacks on sync
- Malicious plugins accessing unauthorized data
- Data exfiltration through side channels

### Out of Scope

- Physical access to unlocked device
- Compromised operating system (rootkits)
- Keyloggers and screen capture malware
- Social engineering attacks

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.x.x   | Yes       |
| < 1.0   | No        |

## Security Updates

Security patches are released as soon as possible after verification. Subscribe to releases to be notified.

## Best Practices for Users

1. **Use a strong passphrase** for vault encryption
2. **Keep Forge updated** to the latest version
3. **Review plugin permissions** before enabling
4. **Use trusted relay servers** for sync (or self-host)
5. **Enable OS disk encryption** as additional protection

## Acknowledgments

We thank security researchers who responsibly disclose vulnerabilities. Contributors will be acknowledged (with permission) in release notes.
