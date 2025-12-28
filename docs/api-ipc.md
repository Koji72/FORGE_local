# Forge v1.0 - API IPC Reference

Este documento define todos los contratos de la API IPC entre el frontend (Svelte) y el backend (Rust/Tauri).

---

## Convenciones Generales

### Formato de Comando

```
v1.<domain>.<action>
```

### Respuesta Exitosa

```json
{
  "ok": true,
  "data": { ... }
}
```

### Respuesta de Error

```json
{
  "ok": false,
  "error": {
    "code": "ERR_...",
    "message": "Human-readable message",
    "details": { ... }
  }
}
```

### Tipos Comunes

```typescript
type UUID = string;           // UUIDv7 format
type Timestamp = number;      // Epoch milliseconds UTC
type TaskStatus = 'inbox' | 'next' | 'waiting' | 'someday' | 'done' | 'archived';
type NoteType = 'note' | 'meeting' | 'journal' | 'reference';
type GoalStatus = 'active' | 'completed' | 'paused' | 'archived';
type GoalHorizon = 'short' | 'medium' | 'long' | 'vision';
type EnergyLevel = 'low' | 'medium' | 'high';
type EntityType = 'task' | 'note' | 'goal';
```

---

## Vault

### `v1.vault.get_info`

Obtiene informacion del vault actual.

**Input:** `{}`

**Output:**
```json
{
  "vault_id": "UUID",
  "db_path": "/path/to/vault.db",
  "encrypted": true,
  "created_at": 1703779200000,
  "size_bytes": 1048576,
  "entity_counts": {
    "tasks": 150,
    "notes": 75,
    "goals": 10
  }
}
```

---

### `v1.vault.export`

Exporta el vault a un archivo.

**Input:**
```json
{
  "format": "json" | "markdown" | "sqlite",
  "path": "/path/to/export",
  "include_deleted": false,
  "entities": ["tasks", "notes", "goals"]
}
```

**Output:**
```json
{
  "path": "/path/to/export/forge-export-20251228.json",
  "size_bytes": 524288,
  "entity_counts": {
    "tasks": 150,
    "notes": 75,
    "goals": 10
  }
}
```

---

### `v1.vault.import`

Importa datos al vault.

**Input:**
```json
{
  "path": "/path/to/import.json",
  "format": "json" | "markdown",
  "merge_strategy": "skip_existing" | "overwrite" | "create_new"
}
```

**Output:**
```json
{
  "imported_counts": {
    "tasks": 50,
    "notes": 25,
    "goals": 5
  },
  "skipped_counts": {
    "tasks": 10,
    "notes": 5,
    "goals": 0
  },
  "errors": []
}
```

---

### `v1.vault.backup_now`

Crea un backup inmediato.

**Input:** `{}`

**Output:**
```json
{
  "backup_id": "UUID",
  "path": "/path/to/backups/forge-backup-20251228-143022.db",
  "size_bytes": 1048576,
  "checksum": "sha256:abc123..."
}
```

---

### `v1.vault.rotate_backups`

Rota backups segun politica configurada.

**Input:** `{}`

**Output:**
```json
{
  "kept": 5,
  "deleted": 3,
  "deleted_paths": [...]
}
```

---

## Tasks

### `v1.tasks.create`

Crea una nueva tarea.

**Input:**
```json
{
  "title": "Task title",
  "description_md": "Optional markdown description",
  "status": "inbox",
  "priority": 2,
  "due_at": 1703779200000,
  "scheduled_at": null,
  "parent_task_id": null,
  "goal_id": null,
  "context_tags": ["@work", "@computer"],
  "energy_level": "medium",
  "time_estimate_min": 30,
  "recurrence_rule": null
}
```

**Output:**
```json
{
  "id": "UUID",
  "title": "Task title",
  "status": "inbox",
  "created_at": 1703779200000,
  "updated_at": 1703779200000
}
```

---

### `v1.tasks.update`

Actualiza una tarea existente.

**Input:**
```json
{
  "id": "UUID",
  "title": "Updated title",
  "status": "next",
  "priority": 1
}
```

**Output:**
```json
{
  "id": "UUID",
  "updated_at": 1703779200000,
  "changes": ["title", "status", "priority"]
}
```

---

### `v1.tasks.list`

Lista tareas con filtros opcionales.

**Input:**
```json
{
  "status": ["inbox", "next"],
  "goal_id": null,
  "due_before": 1703865600000,
  "due_after": null,
  "context_tags": ["@work"],
  "include_deleted": false,
  "order_by": "due_at",
  "order_dir": "asc",
  "limit": 50,
  "offset": 0
}
```

**Output:**
```json
{
  "tasks": [
    {
      "id": "UUID",
      "title": "Task title",
      "description_md": "...",
      "status": "inbox",
      "priority": 2,
      "priority_score": 75.5,
      "due_at": 1703779200000,
      "scheduled_at": null,
      "completed_at": null,
      "parent_task_id": null,
      "goal_id": null,
      "context_tags": ["@work"],
      "energy_level": "medium",
      "time_estimate_min": 30,
      "recurrence_rule": null,
      "created_at": 1703779200000,
      "updated_at": 1703779200000
    }
  ],
  "total": 150,
  "has_more": true
}
```

---

### `v1.tasks.get`

Obtiene una tarea por ID.

**Input:**
```json
{
  "id": "UUID"
}
```

**Output:**
```json
{
  "id": "UUID",
  "title": "Task title",
  ...
}
```

**Errors:**
- `ERR_NOT_FOUND` - Task not found

---

### `v1.tasks.delete`

Elimina una tarea (soft delete).

**Input:**
```json
{
  "id": "UUID",
  "hard_delete": false
}
```

**Output:**
```json
{
  "id": "UUID",
  "deleted_at": 1703779200000
}
```

---

## Notes

### `v1.notes.create`

**Input:**
```json
{
  "title": "Note title",
  "content_md": "# Markdown content",
  "note_type": "note",
  "tags": ["project-x", "ideas"],
  "pinned": false
}
```

**Output:**
```json
{
  "id": "UUID",
  "title": "Note title",
  "created_at": 1703779200000
}
```

---

### `v1.notes.update`

**Input:**
```json
{
  "id": "UUID",
  "title": "Updated title",
  "content_md": "Updated content",
  "pinned": true
}
```

**Output:**
```json
{
  "id": "UUID",
  "updated_at": 1703779200000,
  "changes": ["title", "content_md", "pinned"]
}
```

---

### `v1.notes.list`

**Input:**
```json
{
  "note_type": ["note", "meeting"],
  "tags": ["project-x"],
  "pinned_only": false,
  "include_deleted": false,
  "order_by": "updated_at",
  "order_dir": "desc",
  "limit": 50,
  "offset": 0
}
```

**Output:**
```json
{
  "notes": [...],
  "total": 75,
  "has_more": false
}
```

---

### `v1.notes.get`

**Input:** `{ "id": "UUID" }`

**Output:** Full note object

---

### `v1.notes.delete`

**Input:** `{ "id": "UUID", "hard_delete": false }`

**Output:** `{ "id": "UUID", "deleted_at": ... }`

---

## Goals

### `v1.goals.create`

**Input:**
```json
{
  "title": "Goal title",
  "description_md": "Goal description",
  "status": "active",
  "horizon": "medium",
  "target_date": 1735689600000,
  "parent_goal_id": null
}
```

**Output:**
```json
{
  "id": "UUID",
  "title": "Goal title",
  "created_at": 1703779200000
}
```

---

### `v1.goals.update`

**Input:**
```json
{
  "id": "UUID",
  "progress_percent": 50,
  "status": "active"
}
```

**Output:**
```json
{
  "id": "UUID",
  "updated_at": 1703779200000
}
```

---

### `v1.goals.list`

**Input:**
```json
{
  "status": ["active"],
  "horizon": ["medium", "long"],
  "include_deleted": false,
  "limit": 50,
  "offset": 0
}
```

**Output:**
```json
{
  "goals": [...],
  "total": 10,
  "has_more": false
}
```

---

### `v1.goals.get` / `v1.goals.delete`

Similar a tasks/notes.

---

## Links

### `v1.links.create`

Crea un enlace entre dos entidades.

**Input:**
```json
{
  "from_type": "note",
  "from_id": "UUID",
  "to_type": "task",
  "to_id": "UUID",
  "label": "related_to"
}
```

**Output:**
```json
{
  "id": "UUID",
  "created_at": 1703779200000
}
```

---

### `v1.links.list_by_entity`

Lista enlaces de una entidad.

**Input:**
```json
{
  "entity_type": "note",
  "entity_id": "UUID",
  "direction": "both" | "from" | "to"
}
```

**Output:**
```json
{
  "links": [
    {
      "id": "UUID",
      "from_type": "note",
      "from_id": "UUID",
      "to_type": "task",
      "to_id": "UUID",
      "label": "related_to",
      "created_at": 1703779200000
    }
  ]
}
```

---

### `v1.links.delete`

**Input:** `{ "id": "UUID" }`

**Output:** `{ "deleted": true }`

---

## Search

### `v1.search.fts`

Busqueda full-text.

**Input:**
```json
{
  "query": "meeting notes project",
  "entity_types": ["notes", "tasks"],
  "limit": 20
}
```

**Output:**
```json
{
  "results": [
    {
      "entity_type": "note",
      "entity_id": "UUID",
      "title": "Meeting notes",
      "snippet": "...discussed the **project**...",
      "score": 15.5
    }
  ],
  "total": 5,
  "query_time_ms": 12
}
```

---

### `v1.search.semantic`

Busqueda semantica por embeddings.

**Input:**
```json
{
  "query": "ideas about improving productivity",
  "entity_types": ["notes", "tasks"],
  "limit": 10,
  "min_score": 0.5
}
```

**Output:**
```json
{
  "results": [
    {
      "entity_type": "note",
      "entity_id": "UUID",
      "title": "Productivity tips",
      "similarity": 0.87
    }
  ],
  "query_time_ms": 45,
  "embedding_model": "nomic-embed-text"
}
```

---

### `v1.search.hybrid`

Busqueda combinada FTS + semantica.

**Input:**
```json
{
  "query": "project deadlines",
  "entity_types": ["tasks", "notes"],
  "limit": 20,
  "fts_weight": 0.4,
  "semantic_weight": 0.6
}
```

**Output:**
```json
{
  "results": [
    {
      "entity_type": "task",
      "entity_id": "UUID",
      "title": "Review project deadlines",
      "combined_score": 0.82,
      "fts_score": 12.3,
      "semantic_score": 0.79
    }
  ],
  "query_time_ms": 58
}
```

---

## Agents

### `v1.agents.run`

Ejecuta un agente.

**Input:**
```json
{
  "agent_type": "prioritize_inbox",
  "context": {
    "task_ids": ["UUID1", "UUID2"],
    "criteria": "urgency and importance"
  },
  "options": {
    "timeout_ms": 30000,
    "dry_run": false
  }
}
```

**Output:**
```json
{
  "run_id": "UUID",
  "status": "running",
  "started_at": 1703779200000
}
```

---

### `v1.agents.get_run`

Obtiene estado de un run.

**Input:** `{ "run_id": "UUID" }`

**Output:**
```json
{
  "id": "UUID",
  "agent_type": "prioritize_inbox",
  "status": "completed",
  "input_json": {...},
  "output_json": {
    "ranked_tasks": [
      {
        "task_id": "UUID",
        "priority_score": 85,
        "reason": "Due tomorrow, high importance"
      }
    ]
  },
  "error_json": null,
  "tokens_used": 1250,
  "duration_ms": 3200,
  "started_at": 1703779200000,
  "completed_at": 1703779203200
}
```

---

### `v1.agents.cancel_run`

Cancela un run en progreso.

**Input:** `{ "run_id": "UUID" }`

**Output:** `{ "cancelled": true }`

---

### `v1.agents.list_runs`

Lista historial de runs.

**Input:**
```json
{
  "agent_type": null,
  "status": ["completed", "failed"],
  "limit": 20,
  "offset": 0
}
```

**Output:**
```json
{
  "runs": [...],
  "total": 150
}
```

---

## Sync

### `v1.sync.enable`

Habilita sincronizacion.

**Input:**
```json
{
  "relay_url": "wss://relay.example.com",
  "passphrase": "user-provided-passphrase"
}
```

**Output:**
```json
{
  "enabled": true,
  "vault_id": "UUID",
  "device_id": "UUID",
  "relay_connected": true
}
```

---

### `v1.sync.disable`

Deshabilita sincronizacion.

**Input:** `{}`

**Output:** `{ "disabled": true }`

---

### `v1.sync.status`

Obtiene estado de sync.

**Input:** `{}`

**Output:**
```json
{
  "enabled": true,
  "relay_url": "wss://relay.example.com",
  "relay_connected": true,
  "vault_id": "UUID",
  "device_id": "UUID",
  "last_push_at": 1703779200000,
  "last_pull_at": 1703779100000,
  "pending_changes": 3,
  "devices": [
    {
      "device_id": "UUID",
      "device_name": "MacBook Pro",
      "last_seen_at": 1703779200000,
      "is_current": true
    }
  ]
}
```

---

### `v1.sync.push_pull`

Fuerza sincronizacion inmediata.

**Input:** `{}`

**Output:**
```json
{
  "pushed": 5,
  "pulled": 3,
  "conflicts_resolved": 0,
  "duration_ms": 250
}
```

---

### `v1.sync.rekey`

Rota claves de cifrado.

**Input:**
```json
{
  "new_passphrase": "new-passphrase"
}
```

**Output:**
```json
{
  "key_version": 2,
  "rekeyed_at": 1703779200000
}
```

---

### `v1.sync.reset_device`

Resetea estado de sync del dispositivo.

**Input:** `{ "confirm": true }`

**Output:** `{ "reset": true }`

---

## Plugins

### `v1.plugins.list`

Lista plugins instalados.

**Input:** `{}`

**Output:**
```json
{
  "plugins": [
    {
      "plugin_id": "com.forge.example",
      "name": "Example Plugin",
      "version": "1.0.0",
      "enabled": true,
      "permissions": ["db_read", "db_write"],
      "signature_verified": true,
      "installed_at": 1703779200000
    }
  ]
}
```

---

### `v1.plugins.install`

Instala un plugin desde archivo.

**Input:**
```json
{
  "path": "/path/to/plugin.zip",
  "verify_signature": true
}
```

**Output:**
```json
{
  "plugin_id": "com.forge.example",
  "version": "1.0.0",
  "permissions_requested": ["db_read", "db_write", "network_loopback"],
  "requires_approval": true
}
```

---

### `v1.plugins.enable` / `v1.plugins.disable`

**Input:** `{ "plugin_id": "com.forge.example" }`

**Output:** `{ "enabled": true }` / `{ "enabled": false }`

---

### `v1.plugins.permissions`

Ver y aprobar permisos.

**Input:**
```json
{
  "plugin_id": "com.forge.example",
  "action": "view" | "approve" | "revoke",
  "permissions": ["network_external"]
}
```

**Output:**
```json
{
  "plugin_id": "com.forge.example",
  "permissions": {
    "approved": ["db_read", "db_write"],
    "pending": ["network_external"],
    "revoked": []
  }
}
```

---

### `v1.plugins.uninstall`

**Input:** `{ "plugin_id": "com.forge.example" }`

**Output:** `{ "uninstalled": true }`

---

## Events (Push desde Backend)

Los eventos se emiten via Tauri events y la UI los suscribe.

### `v1.events.db_changed`

```json
{
  "entity_type": "task",
  "entity_id": "UUID",
  "operation": "update",
  "changed_fields": ["status", "priority"]
}
```

### `v1.events.agent_progress`

```json
{
  "run_id": "UUID",
  "progress_percent": 50,
  "current_step": "Analyzing tasks",
  "tokens_so_far": 500
}
```

### `v1.events.sync_state`

```json
{
  "state": "syncing" | "idle" | "error",
  "pending_changes": 5,
  "last_error": null
}
```

### `v1.events.toast`

```json
{
  "level": "info" | "warning" | "error",
  "title": "Sync completed",
  "message": "5 changes pushed, 3 pulled",
  "duration_ms": 5000
}
```

### `v1.events.plugin_state`

```json
{
  "plugin_id": "com.forge.example",
  "state": "loaded" | "unloaded" | "error",
  "error": null
}
```

---

## Settings

### `v1.settings.get`

**Input:** `{ "key": "llm.model.chat" }`

**Output:** `{ "key": "llm.model.chat", "value": "llama3" }`

### `v1.settings.set`

**Input:** `{ "key": "llm.model.chat", "value": "mistral" }`

**Output:** `{ "key": "llm.model.chat", "updated": true }`

### `v1.settings.get_all`

**Input:** `{}`

**Output:**
```json
{
  "settings": {
    "llm.provider": "ollama",
    "llm.model.chat": "llama3",
    ...
  }
}
```

---

## Error Codes Reference

| Code | Description |
|------|-------------|
| `ERR_VALIDATION` | Input validation failed |
| `ERR_NOT_FOUND` | Entity not found |
| `ERR_CONFLICT` | Conflict (e.g., duplicate) |
| `ERR_DB_IO` | Database I/O error |
| `ERR_DB_MIGRATION` | Migration error |
| `ERR_DB_LOCKED` | Database locked |
| `ERR_INDEXING` | Indexing error |
| `ERR_LLM_UNAVAILABLE` | LLM not available |
| `ERR_LLM_TIMEOUT` | LLM timeout |
| `ERR_LLM_BAD_OUTPUT` | LLM output invalid |
| `ERR_SYNC_DISABLED` | Sync not enabled |
| `ERR_SYNC_CRYPTO` | Crypto error in sync |
| `ERR_SYNC_TRANSPORT` | Transport error in sync |
| `ERR_PLUGIN_PERMISSION` | Plugin lacks permission |
| `ERR_PLUGIN_INVALID_SIGNATURE` | Plugin signature invalid |
| `ERR_PLUGIN_RUNTIME` | Plugin runtime error |
| `ERR_INTERNAL` | Internal server error |
