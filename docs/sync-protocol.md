# Forge v1.0 - Sync Protocol Specification

Este documento describe el protocolo de sincronizacion E2EE entre dispositivos Forge via relay WebSocket.

---

## 1) Arquitectura General

```
+-------------+       E2EE       +-------------+       E2EE       +-------------+
|  Device A   | <--------------> |    Relay    | <--------------> |  Device B   |
|  (Client)   |    WebSocket     |   Server    |    WebSocket     |  (Client)   |
+-------------+                  +-------------+                  +-------------+
      |                                |                                |
      v                                v                                v
  Local DB                    Encrypted Store               Local DB
  (SQLCipher)                 (Opaque Blobs)               (SQLCipher)
```

### Principios

1. **Zero-knowledge relay**: El relay solo ve blobs cifrados y metadata minima.
2. **E2EE obligatorio**: Todo payload de sync esta cifrado con claves derivadas del usuario.
3. **CRDT-based**: Usamos `yrs` (Yjs Rust) para resolver conflictos automaticamente.
4. **Materializacion local**: CRDT se materializa a SQL para queries eficientes.

---

## 2) Identidades y Claves

### 2.1 Vault Identity

```
vault_id: UUIDv7 (generado al crear vault, inmutable)
```

### 2.2 Device Identity

```
device_id: UUIDv7 (generado al primer sync enable)
device_name: String (user-provided, e.g., "MacBook Pro")
```

### 2.3 Key Derivation

```
User Passphrase
      |
      v (Argon2id, memory=64MB, iterations=3, parallelism=4)
Master Key (MK) - 32 bytes
      |
      +---> HKDF("forge-sync-enc") --> Encryption Key (EK) - 32 bytes
      |
      +---> HKDF("forge-sync-auth") --> Auth Key (AK) - 32 bytes
```

### 2.4 Device Keys

Generados localmente por cada dispositivo:

```
Ed25519 Signing Key Pair:
  - sk_sign (64 bytes, secret)
  - pk_sign (32 bytes, public, shared with relay and peers)

X25519 Encryption Key Pair (derived from Ed25519):
  - sk_enc (32 bytes, secret)
  - pk_enc (32 bytes, public)
```

---

## 3) Formato de Mensajes (Wire Protocol)

### 3.1 Frame Structure

Todos los mensajes usan el siguiente formato binario:

```
+--------+--------+--------+------------------+
| Type   | Flags  | Length | Payload          |
| 1 byte | 1 byte | 4 bytes| Variable         |
+--------+--------+--------+------------------+
         |        |
         |        +-- Big-endian uint32
         +----------- Reserved for future use
```

### 3.2 Message Types

| Type | Name | Direction | Description |
|------|------|-----------|-------------|
| 0x01 | HELLO | C -> S | Initial handshake |
| 0x02 | WELCOME | S -> C | Handshake response |
| 0x03 | PUSH_DELTA | C -> S | Push encrypted CRDT delta |
| 0x04 | PUSH_ACK | S -> C | Acknowledge push |
| 0x05 | PULL_REQUEST | C -> S | Request deltas since cursor |
| 0x06 | PULL_RESPONSE | S -> C | Batch of deltas |
| 0x07 | HEARTBEAT | C <-> S | Keep-alive |
| 0x08 | ERROR | S -> C | Error response |
| 0x09 | GOODBYE | C -> S | Clean disconnect |

---

## 4) Handshake (HELLO/WELCOME)

### 4.1 HELLO Message

```json
{
  "protocol_version": 1,
  "vault_id": "UUIDv7",
  "device_id": "UUIDv7",
  "device_name": "MacBook Pro",
  "pk_sign": "base64(32 bytes)",
  "pk_enc": "base64(32 bytes)",
  "capabilities": ["crdt_v1", "compress_zstd"],
  "client_time": 1703779200000
}
```

**Nota**: El payload de HELLO no esta cifrado (necesario para establecer sesion).

### 4.2 WELCOME Message

```json
{
  "session_id": "UUIDv7",
  "server_time": 1703779200500,
  "your_cursor": 12345,
  "pending_deltas": 3,
  "other_devices": [
    {
      "device_id": "UUIDv7",
      "device_name": "iPhone",
      "last_seen_at": 1703779100000
    }
  ]
}
```

### 4.3 Handshake Validation

El relay valida:
1. `protocol_version` es soportada
2. `vault_id` existe o se crea
3. `pk_sign` es valido Ed25519 public key
4. Firma del mensaje con `sk_sign` (en header adicional)

---

## 5) Cifrado de Deltas

### 5.1 Estructura de Delta Cifrada

```
+------------------+------------------------+
| Header (clear)   | Encrypted Payload      |
| 48 bytes         | Variable               |
+------------------+------------------------+
```

**Header (48 bytes):**
```
- nonce: 24 bytes (XChaCha20-Poly1305)
- sender_device_id: 16 bytes (UUID bytes)
- sequence: 8 bytes (big-endian uint64)
```

**Encrypted Payload:**
```
XChaCha20-Poly1305(
  key: EK,
  nonce: header.nonce,
  aad: header,
  plaintext: Delta
)
```

### 5.2 Estructura de Delta (Plaintext)

```json
{
  "seq": 12345,
  "timestamp": 1703779200000,
  "entity_updates": [
    {
      "entity_type": "task",
      "entity_id": "UUIDv7",
      "crdt_update": "base64(yrs update bytes)"
    }
  ],
  "checksum": "sha256 of entity_updates"
}
```

---

## 6) Push Flow

### 6.1 PUSH_DELTA

Cliente envia delta cifrada al relay.

```
Frame:
  Type: 0x03
  Payload: [Header(48) | EncryptedDelta]
```

### 6.2 PUSH_ACK

Relay confirma recepcion.

```json
{
  "sequence": 12345,
  "cursor": 67890,
  "received_at": 1703779200500
}
```

### 6.3 Push Ordering

- Cliente mantiene `sequence` monotonicamente creciente.
- Si relay detecta gap en sequence, responde con ERROR y cursor esperado.
- Cliente debe re-sync si hay gap.

---

## 7) Pull Flow

### 7.1 PULL_REQUEST

```json
{
  "since_cursor": 67880,
  "limit": 100,
  "include_own": false
}
```

### 7.2 PULL_RESPONSE

```json
{
  "deltas": [
    {
      "cursor": 67881,
      "sender_device_id": "UUIDv7",
      "timestamp": 1703779150000,
      "encrypted_delta": "base64(...)"
    }
  ],
  "has_more": false,
  "latest_cursor": 67890
}
```

### 7.3 Decryption

Cliente descifra cada `encrypted_delta` con su copia local de `EK`.

---

## 8) CRDT Merge y Materializacion

### 8.1 Yrs (Yjs) Doc Structure

Cada entidad tiene un `Y.Map` correspondiente:

```
YDoc
  |-- tasks (Y.Map)
  |     |-- {task_id} (Y.Map)
  |           |-- title (Y.Text)
  |           |-- status (Y.Text/LWW)
  |           |-- priority (Y.Number/LWW)
  |           |-- ...
  |
  |-- notes (Y.Map)
  |     |-- {note_id} (Y.Map)
  |           |-- title (Y.Text)
  |           |-- content_md (Y.Text)
  |           |-- ...
  |
  |-- goals (Y.Map)
        |-- ...
```

### 8.2 Materializacion a SQL

Despues de aplicar updates de CRDT:

```rust
fn materialize_to_sql(ydoc: &YDoc, db: &Connection) -> Result<()> {
    for (id, task_map) in ydoc.get_map("tasks").iter() {
        db.execute(
            "INSERT OR REPLACE INTO tasks (...) VALUES (...)",
            params![...]
        )?;
    }
    // Similar para notes, goals
}
```

### 8.3 Conflict Resolution

| Campo | Estrategia |
|-------|-----------|
| `title` | Y.Text (merge caracteres) |
| `content_md` | Y.Text (merge caracteres) |
| `status` | LWW (last writer wins by timestamp) |
| `priority` | LWW |
| `priority_score` | **No sincronizado** (derivado local) |
| `updated_at` | Max timestamp wins |
| `deleted_at` | Any non-null wins |

---

## 9) Relay Storage

### 9.1 Schema (Relay DB)

```sql
CREATE TABLE vaults (
  vault_id TEXT PRIMARY KEY,
  created_at INTEGER NOT NULL,
  last_activity_at INTEGER NOT NULL
);

CREATE TABLE devices (
  device_id TEXT PRIMARY KEY,
  vault_id TEXT NOT NULL REFERENCES vaults(vault_id),
  pk_sign BLOB NOT NULL,
  device_name TEXT,
  added_at INTEGER NOT NULL,
  last_seen_at INTEGER
);

CREATE TABLE deltas (
  cursor INTEGER PRIMARY KEY AUTOINCREMENT,
  vault_id TEXT NOT NULL,
  sender_device_id TEXT NOT NULL,
  encrypted_delta BLOB NOT NULL,
  received_at INTEGER NOT NULL,
  expires_at INTEGER
);

CREATE INDEX idx_deltas_vault ON deltas(vault_id, cursor);
```

### 9.2 Retention Policy

- Default TTL: 30 dias
- Cleanup job: cada hora elimina deltas expirados
- Configurable por vault (si se implementa accounts)

---

## 10) Anti-Replay Protection

### 10.1 Sequence Numbers

- Cada dispositivo mantiene `sequence` contador local.
- Relay rechaza deltas con `sequence <= last_seen_sequence[device_id]`.

### 10.2 Timestamp Validation

- Relay rechaza deltas con `timestamp` mas de 5 minutos en el futuro.
- Relay rechaza deltas con `timestamp` mas de 7 dias en el pasado.

### 10.3 Seen Delta Cache

Cliente mantiene cache de `(sender_device_id, sequence)` vistos en ventana de 24h.

---

## 11) Error Handling

### 11.1 ERROR Message

```json
{
  "code": "SYNC_ERR_...",
  "message": "Human readable",
  "details": {...}
}
```

### 11.2 Error Codes

| Code | Description | Action |
|------|-------------|--------|
| `SYNC_ERR_INVALID_HANDSHAKE` | Malformed HELLO | Reconnect |
| `SYNC_ERR_VAULT_NOT_FOUND` | Vault doesn't exist | Create vault |
| `SYNC_ERR_DEVICE_UNKNOWN` | Device not registered | Re-register |
| `SYNC_ERR_SEQUENCE_GAP` | Missing sequence numbers | Re-sync |
| `SYNC_ERR_TIMESTAMP_FUTURE` | Timestamp too far ahead | Check clock |
| `SYNC_ERR_DECRYPT_FAILED` | Client can't decrypt | Re-key |
| `SYNC_ERR_RATE_LIMITED` | Too many requests | Backoff |

---

## 12) Reconnection y Backoff

### 12.1 Reconnection Strategy

```
Attempt 1: Immediate
Attempt 2: Wait 1s
Attempt 3: Wait 2s
Attempt 4: Wait 4s
Attempt 5: Wait 8s
...
Max wait: 60s
Max attempts: Infinite (user can disable sync)
```

### 12.2 Heartbeat

- Intervalo: 30 segundos
- Timeout: 90 segundos sin respuesta -> reconnect

---

## 13) Security Considerations

### 13.1 Forward Secrecy

Para v1.0: No implementado (misma EK para toda la vida del vault).

Para vNext: Implementar ratchet protocol con session keys efimeras.

### 13.2 Key Rotation

`v1.sync.rekey` permite rotar claves:

1. Deriva nueva EK/AK de nueva passphrase
2. Re-cifra todas las claves locales
3. Notifica a peers via delta especial (encrypted with old + new key)
4. Incrementa `key_version`

### 13.3 Device Revocation

No implementado en v1.0. Workaround: re-key con nueva passphrase y no compartir con dispositivo revocado.

---

## 14) Implementation Checklist

- [ ] Wire protocol parser/serializer
- [ ] XChaCha20-Poly1305 encryption/decryption
- [ ] Ed25519 signing/verification
- [ ] Key derivation (Argon2id + HKDF)
- [ ] Yrs integration for CRDT
- [ ] SQL materialization after CRDT apply
- [ ] WebSocket client with reconnection
- [ ] Sequence number tracking
- [ ] Relay server (separate project or reference impl)
- [ ] Anti-replay cache
- [ ] Heartbeat mechanism
- [ ] Error handling and recovery

---

## Appendix A: Example Flow

```
1. User enables sync with passphrase "my-secret-phrase"
2. Client derives MK, EK, AK from passphrase
3. Client generates Ed25519 keypair if not exists
4. Client opens WebSocket to relay
5. Client sends HELLO with vault_id, device_id, pk_sign
6. Relay responds with WELCOME, includes cursor and pending count
7. Client sends PULL_REQUEST since last cursor
8. Relay responds with PULL_RESPONSE containing encrypted deltas
9. Client decrypts and applies CRDT updates
10. Client materializes CRDT to SQL
11. Client makes local change (e.g., create task)
12. Client creates CRDT update
13. Client encrypts update with EK
14. Client sends PUSH_DELTA
15. Relay stores and responds with PUSH_ACK
16. Other devices receive on their next PULL
```

---

## Appendix B: Test Vectors

### Key Derivation Test

```
Passphrase: "test-passphrase-123"
Salt: 0x0000...0000 (for testing only)
MK: 0x... (32 bytes)
EK: 0x... (32 bytes, via HKDF)
AK: 0x... (32 bytes, via HKDF)
```

### Encryption Test

```
Plaintext: {"seq": 1, "entity_updates": []}
Nonce: 0x... (24 bytes)
EK: 0x... (32 bytes)
Ciphertext: 0x... (variable)
Tag: 0x... (16 bytes)
```

(Test vectors TBD durante implementacion)
