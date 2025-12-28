# Forge v1.0 - Threat Model

Este documento analiza las amenazas de seguridad para Forge y las mitigaciones implementadas.

---

## 1) Scope y Assets

### 1.1 Assets Protegidos

| Asset | Sensitivity | Description |
|-------|-------------|-------------|
| Vault Database | HIGH | SQLite con tareas, notas, objetivos del usuario |
| Passphrase | CRITICAL | Clave maestra para cifrado |
| Device Keys | HIGH | Ed25519/X25519 keypairs |
| Agent Outputs | MEDIUM | Resultados de AI (pueden contener datos) |
| Sync Traffic | HIGH | Deltas CRDT en transito |
| Plugin Code | MEDIUM | Codigo de terceros ejecutandose |

### 1.2 Out of Scope

- Ataques fisicos al hardware
- Compromiso total del OS (rootkit, keylogger kernel-level)
- Side-channel attacks (timing, power analysis)
- Ataques a Ollama/LLM runtime

---

## 2) Threat Actors

| Actor | Capability | Motivation |
|-------|------------|------------|
| Script Kiddie | Low | Opportunistic data theft |
| Malware | Medium | Data exfiltration, ransomware |
| Malicious Plugin | Medium | Data access, persistence |
| Network Attacker | Medium | Traffic interception, MitM |
| Compromised Relay | Medium | Metadata harvesting, DoS |
| Sophisticated Attacker | High | Targeted attack on individual |

---

## 3) STRIDE Analysis

### 3.1 Spoofing

| Threat | Risk | Mitigation |
|--------|------|------------|
| Impersonate another device in sync | HIGH | Ed25519 signatures on all sync messages |
| Fake plugin as legitimate | MEDIUM | Plugin signature verification (Ed25519) |
| Spoof IPC commands | LOW | Tauri IPC only from trusted webview |

### 3.2 Tampering

| Threat | Risk | Mitigation |
|--------|------|------------|
| Modify vault DB on disk | HIGH | SQLCipher encryption (AES-256) |
| Tamper sync traffic | HIGH | XChaCha20-Poly1305 AEAD |
| Modify HNSW index | MEDIUM | Index encrypted with vault key |
| Tamper plugin at rest | MEDIUM | Signature verification on load |

### 3.3 Repudiation

| Threat | Risk | Mitigation |
|--------|------|------------|
| Deny agent action | LOW | `agent_runs` table logs all executions |
| Deny sync origin | LOW | Device signatures on deltas |

### 3.4 Information Disclosure

| Threat | Risk | Mitigation |
|--------|------|------------|
| Read vault without passphrase | HIGH | SQLCipher + Argon2id key derivation |
| Intercept sync traffic | HIGH | E2EE (relay sees only ciphertext) |
| Extract keys from memory | MEDIUM | Zeroize sensitive data; OS memory protection |
| Log sensitive data | MEDIUM | No PII in logs; structured logging |
| Plugin exfiltrate data | MEDIUM | Permission system; no network by default |

### 3.5 Denial of Service

| Threat | Risk | Mitigation |
|--------|------|------------|
| Corrupt database | MEDIUM | WAL mode; automatic backups |
| Exhaust disk with embeddings | LOW | Configurable limits; cleanup policy |
| Relay DoS | LOW | Rate limiting; client reconnection |
| Plugin infinite loop | MEDIUM | Execution timeouts; sandbox |

### 3.6 Elevation of Privilege

| Threat | Risk | Mitigation |
|--------|------|------------|
| XSS in webview | MEDIUM | CSP; input sanitization; IPC allowlist |
| Plugin escape sandbox | MEDIUM | Capability-based permissions; no arbitrary FS/net |
| IPC command injection | LOW | Strict input validation; typed DTOs |

---

## 4) Attack Trees

### 4.1 Steal Vault Data

```
[Steal Vault Data]
  |
  +-- [1] Obtain passphrase
  |     +-- [1.1] Phishing (out of scope)
  |     +-- [1.2] Keylogger (out of scope - OS level)
  |     +-- [1.3] Memory dump MITIGATED: zeroize
  |
  +-- [2] Access encrypted vault
  |     +-- [2.1] Copy .db file
  |           +-- Still need passphrase MITIGATED
  |
  +-- [3] Intercept sync traffic
  |     +-- [3.1] MitM relay
  |           +-- E2EE prevents content access MITIGATED
  |
  +-- [4] Malicious plugin
        +-- [4.1] Install unsigned plugin
        |     +-- Signature verification MITIGATED
        +-- [4.2] Exploit db_read permission
              +-- User must approve MITIGATED
```

### 4.2 Compromise Sync

```
[Compromise Sync]
  |
  +-- [1] Impersonate device
  |     +-- Ed25519 signatures required MITIGATED
  |
  +-- [2] Replay old deltas
  |     +-- Sequence + timestamp validation MITIGATED
  |
  +-- [3] Inject malicious delta
  |     +-- AEAD authentication fails MITIGATED
  |
  +-- [4] Relay stores plaintext
        +-- Zero-knowledge design MITIGATED
```

---

## 5) Security Controls

### 5.1 Cryptographic Controls

| Control | Implementation | Status |
|---------|----------------|--------|
| Vault encryption | SQLCipher AES-256-CBC | Required |
| Key derivation | Argon2id (64MB, 3 iter, 4 parallel) | Required |
| Sync encryption | XChaCha20-Poly1305 | Required |
| Signatures | Ed25519 | Required |
| Key exchange | X25519 (derived from Ed25519) | Required |

### 5.2 Access Controls

| Control | Implementation | Status |
|---------|----------------|--------|
| Plugin permissions | Capability-based, user approval | Required |
| IPC allowlist | Tauri command allowlist | Required |
| CSP | Strict policy, no inline scripts | Required |
| Filesystem access | Plugins: sandbox only | Required |
| Network access | Plugins: blocked by default | Required |

### 5.3 Operational Controls

| Control | Implementation | Status |
|---------|----------------|--------|
| Automatic backups | Configurable rotation | Required |
| Audit logging | `agent_runs`, structured logs | Required |
| Update mechanism | Signed updates via Tauri | Required |
| Dependency audit | `cargo audit` in CI | Required |

---

## 6) Recommendations

### 6.1 v1.0 (Must Have)

- [x] SQLCipher encryption for vault
- [x] Argon2id key derivation with strong parameters
- [x] E2EE for sync with XChaCha20-Poly1305
- [x] Plugin signature verification
- [x] Plugin permission system
- [x] CSP for webview
- [x] IPC command validation
- [x] Automatic backup rotation
- [x] Audit logging

### 6.2 v1.1 (Should Have)

- [ ] Hardware key support (YubiKey for passphrase derivation)
- [ ] Biometric unlock (where supported by OS)
- [ ] Forward secrecy in sync protocol
- [ ] Device revocation mechanism
- [ ] Plugin sandboxing with WASM

### 6.3 vNext (Nice to Have)

- [ ] Memory encryption at runtime
- [ ] Secure enclave integration (Apple T2, TPM)
- [ ] Post-quantum cryptography migration path

---

## 7) Incident Response

### 7.1 Passphrase Compromise

1. User rotates passphrase via `v1.sync.rekey`
2. All devices must re-authenticate
3. Consider vault export + fresh start if severity high

### 7.2 Device Loss

1. Remove device from `sync_devices` (future: device revocation)
2. Rotate passphrase if device was unlocked
3. Consider data exposure scope

### 7.3 Malicious Plugin Discovered

1. Disable plugin immediately
2. Uninstall and delete plugin files
3. Audit `agent_runs` and logs for activity
4. Consider vault integrity check

---

## 8) Compliance Notes

### 8.1 GDPR Considerations

- All data local-first, user controlled
- No telemetry by default
- Export/delete functionality available
- No data sent to Anthropic/OpenAI (unless user configures external LLM)

### 8.2 Data Residency

- Vault stays on user's device
- Sync relay can be self-hosted
- User controls all data locations

---

## 9) Security Testing Checklist

### 9.1 Pre-Release

- [ ] Fuzz IPC inputs
- [ ] Test SQLCipher with wrong passphrase
- [ ] Test sync with tampered messages
- [ ] Test plugin with elevated permission requests
- [ ] Verify CSP blocks inline scripts
- [ ] Verify no secrets in logs
- [ ] Run `cargo audit`
- [ ] Review dependency tree

### 9.2 Periodic

- [ ] Dependency audit (monthly)
- [ ] Penetration test (annually)
- [ ] Crypto library updates (as released)
- [ ] Review new attack vectors

---

## Appendix: CSP Configuration

```
default-src 'self';
script-src 'self';
style-src 'self' 'unsafe-inline';
img-src 'self' data:;
font-src 'self';
connect-src 'self' http://127.0.0.1:11434;
frame-src 'none';
object-src 'none';
base-uri 'self';
form-action 'self';
```

**Notes:**
- `'unsafe-inline'` for styles only (Svelte requirement)
- `connect-src` includes Ollama loopback
- No external resources allowed
