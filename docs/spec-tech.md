# Forge v1.0 - Technical Specification

**Proyecto:** Forge (Vectal-Local)
**Tipo:** Aplicacion local-first con AI agents + sync opcional E2EE
**Version Spec:** 1.0 (28/12/2025)
**Audiencia:** Lead devs, backend (Rust), UI (Svelte), infra, QA, seguridad

---

## 0) Portada y Control de Cambios

### 0.1 Objetivos de Producto (Definitivos)

- **Offline-complete:** todo funciona sin conexion.
- **Local-first:** datos y ejecucion por defecto en el dispositivo.
- **Privacidad:** no hay telemetria por defecto. Sync opcional siempre E2EE.
- **Extensible:** plugins con permisos, sin romper seguridad.
- **Rendimiento nativo:** Tauri + Rust, sin Electron.

### 0.2 No Objetivos (v1.0)

- Multiusuario con roles y ACL a nivel servidor.
- Colaboracion en tiempo real tipo Google Docs (solo sync asincrono por CRDT).
- Hosting cloud obligatorio o cuentas centrales.

---

## 1) Requisitos Funcionales y No Funcionales

### 1.1 Requisitos Funcionales (FR)

| ID | Requisito |
|----|-----------|
| FR-01 | **Vault local:** CRUD de tareas, notas, objetivos, enlaces. |
| FR-02 | **Busqueda:** FTS + busqueda semantica local (embeddings + ANN). |
| FR-03 | **Agentes:** ejecutar agentes que leen/escriben en el vault; runs auditables. |
| FR-04 | **Export/Import:** JSON/SQL/Markdown con integridad y opciones. |
| FR-05 | **Sync opcional:** multi-dispositivo via relay WS, E2EE, CRDT, reconciliacion. |
| FR-06 | **Plugins:** instalar/activar/desactivar, permisos y aislamiento. |
| FR-07 | **Backups:** rotacion automatica + recuperacion. |
| FR-08 | **Configuracion:** modelo LLM, embeddings, sync, cifrado, limites. |

### 1.2 Requisitos No Funcionales (NFR)

| ID | Requisito |
|----|-----------|
| NFR-01 | **Seguridad:** cifrado en reposo + E2EE en transito (si sync). |
| NFR-02 | **Integridad:** transacciones, WAL, migraciones idempotentes. |
| NFR-03 | **Rendimiento:** UI fluida; operaciones comunes < 50 ms; busqueda FTS < 150 ms; ANN < 200 ms para 10k items (target). |
| NFR-04 | **Robustez:** fallos de LLM, DB y sync con recuperacion; no perdida silenciosa. |
| NFR-05 | **Portabilidad:** Windows/macOS/Linux (v1), iOS/Android (vNext). |
| NFR-06 | **Observabilidad local:** logs estructurados, trazas de runs y sync, sin enviar fuera. |

---

## 2) Arquitectura del Sistema

### 2.1 Patron: Monolito Modular Nativo

- **Core (Rust)**: dominio, storage, indexing, agents, sync, plugins.
- **UI (Svelte 5 + SvelteKit)** embebida en **Tauri 2.9+**.
- **LLM runtime local** por loopback (Ollama) con adaptadores.

### 2.2 Mapa de Modulos (Crates)

```
crates/
  forge-domain/      # Entidades de dominio, validaciones
  forge-storage/     # SQLite + SQLCipher, migraciones
  forge-indexing/    # FTS + embeddings + HNSW
  forge-agents/      # Orquestacion + tools + parse estricto
  forge-sync/        # yrs + relay WS + E2EE
  forge-plugins/     # manifest + permisos + sandbox
  forge-api-ipc/     # DTOs, versionado, errores
  forge-app/         # Wiring Tauri commands/events
```

### 2.3 Principios de Diseno

- **Single source of truth**: SQLite cifrado.
- **Materializacion CRDT -> SQL**: CRDT es capa de sync; SQL manda para queries.
- **Parse estricto en agentes**: outputs siempre contra JSON Schema.
- **Fail closed**: si un plugin no tiene permiso, se bloquea.
- **No HTTP server publico**: solo IPC.

---

## 3) Stack Tecnologico

| Componente | Tecnologia |
|------------|------------|
| Backend | Rust 1.82+ stable |
| Framework Desktop | Tauri 2.9.x+ |
| UI | Svelte 5 + SvelteKit 2.x |
| Database | SQLite 3.x + SQLCipher (AES-256) |
| Full-Text Search | FTS5 (integrado) |
| Embeddings | Ollama (fase 1) |
| ANN Index | HNSW embebido (Rust) |
| CRDT | yrs (Yjs Rust) |
| Crypto | libsodium (XChaCha20-Poly1305 + Ed25519) + Argon2id |
| CI/CD | GitHub Actions |
| Testing | cargo test + Playwright (UI) + fuzz (opcional) |
| Logs | JSON line logs (local only) |

---

## 4) Estructura de Repositorio

```
forge/
  crates/
    forge-domain/
    forge-storage/
    forge-indexing/
    forge-agents/
    forge-sync/
    forge-plugins/
    forge-api-ipc/
    forge-app/
  apps/
    forge-ui/              # SvelteKit
  tauri/
    src-tauri/
      Cargo.toml
      tauri.conf.json
      src/main.rs
  docs/
    spec-tech.md           # Este documento
    api-ipc.md
    threat-model.md
    sync-protocol.md
    plugin-dev.md
  schemas/
    agents/                # JSON Schemas para agentes
  migrations/
    0001_init.sql
  .github/workflows/
    ci.yml
    release.yml
```

---

## 5) Modelado de Datos (SQLCipher)

### 5.1 Convenciones

- IDs: **UUIDv7** (texto)
- Timestamps: **epoch ms UTC**
- Soft delete: `deleted_at`
- Migraciones: semver `0001_init.sql`, `0002_...`

### 5.2 Tablas Principales

Ver `migrations/0001_init.sql` para esquema completo.

| Tabla | Descripcion |
|-------|-------------|
| `tasks` | Tareas con status, prioridad, fechas, recurrencia |
| `notes` | Notas markdown con tipos y tags |
| `goals` | Objetivos con horizonte y progreso |
| `links` | Grafo de relaciones entre entidades |
| `agent_runs` | Historial de ejecuciones de agentes |
| `embeddings` | Vectores para busqueda semantica |
| `sync_devices` | Dispositivos registrados para sync |
| `sync_state` | Estado de sincronizacion |
| `plugins` | Plugins instalados |
| `settings` | Configuracion del usuario |

### 5.3 Indices y Limites

- Limite target inicial: 100k tasks/notes combinadas (no hard limit, solo budget).
- ANN: mantener `HNSW` por entidad o indice global con `entity_type` filter.

---

## 6) Indexacion: FTS + Semantica

### 6.1 FTS

- Tablas virtuales FTS5 para notes, tasks, goals
- Triggers automaticos para mantener indices sincronizados
- Busqueda con ranking BM25

### 6.2 Embeddings

- **Fuente:** Ollama embeddings endpoint (fase 1)
- **Modelo default:** `nomic-embed-text` (configurable)
- **Dimension:** detectar del modelo y persistir

**Politica de recalculado:**
- on create/update: encolar job
- procesamiento en background (idle)
- throttle: 1 job concurrente por defecto

### 6.3 ANN (HNSW embebido)

- Persistencia: archivo `forge.hnsw` por vault, cifrado
- Clave: `entity_id` -> vector
- Operacion: `topK(query_vec, filter)` con filtro por `entity_type`

---

## 7) API IPC

Ver `docs/api-ipc.md` para contratos completos.

### 7.1 Convencion General

- Command: `v1.<domain>.<action>`
- Respuesta exito: `{ "ok": true, "data": ... }`
- Respuesta error: `{ "ok": false, "error": { "code": "ERR_...", "message": "...", "details": {...} } }`

### 7.2 Dominios

- `vault` - Info, export, import, backups
- `tasks` - CRUD tareas
- `notes` - CRUD notas
- `goals` - CRUD objetivos
- `links` - Gestionar enlaces entre entidades
- `search` - FTS, semantica, hibrida
- `agents` - Ejecutar y gestionar agentes
- `sync` - Sincronizacion E2EE
- `plugins` - Gestionar plugins

---

## 8) Agentes: Contratos y Ejecucion

### 8.1 Tipos de Agentes v1

1. `prioritize_inbox` - Priorizar tareas del inbox
2. `weekly_review` - Revision semanal
3. `summarize_note` - Resumir nota
4. `extract_tasks_from_note` - Extraer tareas de nota
5. `plan_day` - Planificar dia

### 8.2 Tools Disponibles

- `db.query_tasks(filter)`
- `db.query_notes(filter)`
- `db.update_task(id, patch)`
- `search.fts(query)`
- `search.semantic(query)`
- `time.now()` / `time.today()`
- `vault.settings_get(key)` / `settings_set(key, value)`

### 8.3 Politica de Repair

1. Si output no valida: reintento con prompt de reparacion
2. Si falla: `ERR_LLM_BAD_OUTPUT`, run `failed`

### 8.4 Timeouts y Limites

- Timeout por defecto: 30s (configurable)
- Concurrencia: 1 run simultaneo (configurable a 2)

---

## 9) Seguridad

### 9.1 Cifrado en Reposo

- DB: SQLCipher AES-256
- Passphrase usuario -> Argon2id -> KEK
- KEK envuelve DEK_DB

### 9.2 Cifrado E2EE Sync

- Passphrase -> Argon2id -> MK (master key)
- Device keys: Ed25519 (firma) + X25519 (cifrado)
- Mensajes: XChaCha20-Poly1305 AEAD
- Anti-replay: contador/nonce + timestamp + cache ids

### 9.3 Threat Model

Ver `docs/threat-model.md` para analisis completo.

---

## 10) Sync: Protocolo y Relay

Ver `docs/sync-protocol.md` para detalles.

### 10.1 Modelo

- CRDT: yrs
- Transporte: Relay WebSocket (self-hosted opcional)
- Mensajes: binario cifrado

### 10.2 Identidad

- `vault_id`: UUIDv7 generado al crear vault
- `device_id`: UUIDv7 por dispositivo

---

## 11) Plugins

Ver `docs/plugin-dev.md` para guia de desarrollo.

### 11.1 Permisos (Catalogo v1)

- `db_read`, `db_write`
- `filesystem_export`
- `network_loopback`
- `network_external` (requiere aprobacion)
- `clipboard_read`, `clipboard_write`

---

## 12) Manejo de Errores

### 12.1 Codigos de Error

| Codigo | Descripcion |
|--------|-------------|
| `ERR_VALIDATION` | Error de validacion de datos |
| `ERR_DB_IO` | Error de I/O en base de datos |
| `ERR_DB_MIGRATION` | Error en migracion |
| `ERR_DB_LOCKED` | Base de datos bloqueada |
| `ERR_INDEXING` | Error en indexacion |
| `ERR_LLM_UNAVAILABLE` | LLM no disponible |
| `ERR_LLM_TIMEOUT` | Timeout de LLM |
| `ERR_LLM_BAD_OUTPUT` | Output de LLM invalido |
| `ERR_SYNC_DISABLED` | Sync deshabilitado |
| `ERR_SYNC_CRYPTO` | Error criptografico en sync |
| `ERR_SYNC_TRANSPORT` | Error de transporte en sync |
| `ERR_PLUGIN_PERMISSION` | Plugin sin permisos |
| `ERR_PLUGIN_INVALID_SIGNATURE` | Firma de plugin invalida |
| `ERR_PLUGIN_RUNTIME` | Error de runtime de plugin |

---

## 13) Observabilidad Local

### 13.1 Logs

- Formato: JSON Lines
- Campos: `ts`, `level`, `module`, `event`, `vault_id`, `device_id`, `details`
- Ubicacion: `logs/forge-YYYYMMDD.jsonl`

### 13.2 Auditoria de Runs

- Tabla `agent_runs` guarda input/output y errores

### 13.3 Diagnostico UI

- Estado DB
- Estado indexing (cola)
- Estado LLM (health)
- Estado sync (cursors)
- Lista plugins + permisos

---

## 14) Plan de Pruebas (QA)

### 14.1 Unit Tests (Rust)

- dominio: invariantes (status transitions, timestamps)
- storage: transacciones, migraciones, backups
- agents: parse schema + repair
- sync: cifrado/descifrado, merge determinista

### 14.2 Integration Tests

- Happy path CRUD + search + agent run
- Corrupcion simulada DB -> recovery
- Ollama down -> `ERR_LLM_UNAVAILABLE`
- Sync push/pull con 2 dispositivos simulados

### 14.3 UI E2E (Playwright)

- Crear task -> aparece en lista
- Buscar FTS
- Ejecutar priorizacion
- Export y comprobar archivo
- Enable sync y ver estado

### 14.4 Criterios de Aceptacion (MVP)

- [ ] Vault cifrado y desbloqueo por passphrase
- [ ] CRUD estable
- [ ] FTS funcionando
- [ ] 1 agente funcional con output validado
- [ ] Export JSON/MD
- [ ] Sync E2EE con relay para tasks + notes
- [ ] Plugins instalables con permisos

---

## 15) CI/CD y Release Engineering

### 15.1 CI Pipeline

- lint/format + tests + audit
- build multi-OS
- artefactos firmados en release

### 15.2 Release Pipeline

- Version semver
- Changelog
- Firma
- Auto-update feed

---

## 16) Configuracion

### 16.1 Settings Keys

| Key | Default | Descripcion |
|-----|---------|-------------|
| `llm.provider` | `ollama` | Proveedor LLM |
| `llm.ollama.base_url` | `http://127.0.0.1:11434` | URL base Ollama |
| `llm.model.chat` | `llama3` | Modelo para chat |
| `llm.model.embed` | `nomic-embed-text` | Modelo para embeddings |
| `agents.timeout_ms` | `30000` | Timeout de agentes |
| `indexing.embed_on_idle` | `true` | Embeddings en idle |
| `sync.enabled` | `false` | Sync habilitado |
| `sync.relay_url` | `""` | URL del relay |
| `security.vault_encryption` | `true` | Cifrado vault |
| `backups.keep` | `5` | Backups a mantener |

---

## 17) Roadmap de Implementacion

| Sprint | Objetivo |
|--------|----------|
| 1-2 | Base Tauri + IPC + storage cifrado + migraciones + CRUD |
| 3 | FTS + export/import + backups + diagnostics |
| 4 | Agents (run + schema parse + 1 agente) + agent_runs |
| 5 | Embeddings (via Ollama) + HNSW + busqueda semantica/hibrida |
| 6 | Sync: yrs + relay WS + E2EE + materializacion SQL |
| 7 | Plugins v0: manifest + permisos + enable/disable + firma |
| 8 | Hardening + QA + release pipeline |

---

## Documentos Relacionados

- [API IPC](api-ipc.md) - Contratos completos de la API
- [Sync Protocol](sync-protocol.md) - Protocolo de sincronizacion
- [Threat Model](threat-model.md) - Modelo de amenazas
- [Plugin Development](plugin-dev.md) - Guia de desarrollo de plugins
