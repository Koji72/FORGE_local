# Forge v1.0 - Plugin Development Guide

Esta guia explica como desarrollar plugins para Forge.

---

## 1) Introduccion

Los plugins de Forge permiten extender la funcionalidad de la aplicacion de forma segura y controlada. Cada plugin:

- Declara permisos explicitos en su manifest
- Esta firmado digitalmente (Ed25519)
- Se ejecuta en un sandbox JavaScript
- Solo accede a las APIs autorizadas

---

## 2) Estructura de un Plugin

```
my-plugin/
  manifest.json          # Metadatos y permisos
  main.js                # Codigo principal (bundle)
  schemas/
    input.json           # JSON Schema para inputs (opcional)
    output.json          # JSON Schema para outputs (opcional)
  assets/
    icon.png             # Icono del plugin (64x64)
  README.md              # Documentacion
```

---

## 3) Manifest

### 3.1 Estructura Completa

```json
{
  "plugin_id": "com.example.my-plugin",
  "name": "My Awesome Plugin",
  "version": "1.0.0",
  "description": "A short description of what this plugin does",
  "author": {
    "name": "Developer Name",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "license": "MIT",
  "min_forge_version": "1.0.0",
  "max_forge_version": "2.0.0",

  "permissions": [
    "db_read",
    "db_write"
  ],

  "commands": [
    {
      "name": "my-plugin.doSomething",
      "description": "Does something useful",
      "input_schema": "schemas/input.json",
      "output_schema": "schemas/output.json"
    }
  ],

  "hooks": {
    "onInstall": "onInstall",
    "onEnable": "onEnable",
    "onDisable": "onDisable",
    "onUninstall": "onUninstall"
  },

  "ui": {
    "panels": [
      {
        "id": "my-panel",
        "title": "My Panel",
        "location": "sidebar",
        "component": "MyPanel"
      }
    ],
    "settings": [
      {
        "key": "my-plugin.option1",
        "type": "string",
        "default": "value",
        "label": "Option 1",
        "description": "Description of option"
      }
    ]
  },

  "signature": {
    "algo": "ed25519",
    "public_key": "BASE64_ENCODED_PUBLIC_KEY",
    "signed_hash": "BASE64_ENCODED_SIGNATURE"
  }
}
```

### 3.2 Campos Obligatorios

| Campo | Descripcion |
|-------|-------------|
| `plugin_id` | ID unico (reverse domain notation) |
| `name` | Nombre legible |
| `version` | Semver |
| `min_forge_version` | Version minima de Forge requerida |
| `permissions` | Lista de permisos requeridos |
| `signature` | Firma digital del plugin |

---

## 4) Permisos

### 4.1 Catalogo de Permisos v1

| Permiso | Descripcion | Riesgo |
|---------|-------------|--------|
| `db_read` | Leer tareas, notas, objetivos | Medio |
| `db_write` | Escribir/modificar entidades | Alto |
| `filesystem_export` | Escribir en carpeta de exports | Bajo |
| `network_loopback` | Conexiones a localhost | Bajo |
| `network_external` | Conexiones externas (requiere aprobacion explicita) | Alto |
| `clipboard_read` | Leer portapapeles | Medio |
| `clipboard_write` | Escribir al portapapeles | Bajo |

### 4.2 Solicitar Permisos

Los permisos se declaran en `manifest.json`. El usuario debe aprobarlos al instalar/habilitar.

```json
{
  "permissions": ["db_read", "db_write", "network_loopback"]
}
```

### 4.3 Permisos Denegados

Si un plugin intenta usar una API sin permiso:

```javascript
// Plugin code
const tasks = await forge.db.queryTasks({ status: 'inbox' });
// Throws: PluginPermissionError: db_read permission required
```

---

## 5) API del Plugin

### 5.1 Objeto Global `forge`

Todos los plugins tienen acceso al objeto `forge`:

```javascript
// Disponible en el contexto del plugin
const forge = {
  db: { ... },
  search: { ... },
  settings: { ... },
  ui: { ... },
  log: { ... }
};
```

### 5.2 Database API (`forge.db`)

**Requiere:** `db_read` y/o `db_write`

```javascript
// Leer tareas
const tasks = await forge.db.queryTasks({
  status: ['inbox', 'next'],
  limit: 50
});

// Leer una tarea
const task = await forge.db.getTask('uuid');

// Crear tarea (requiere db_write)
const newTask = await forge.db.createTask({
  title: 'New task',
  status: 'inbox'
});

// Actualizar tarea (requiere db_write)
await forge.db.updateTask('uuid', {
  status: 'done',
  completed_at: Date.now()
});

// Notas y Goals: API similar
const notes = await forge.db.queryNotes({ ... });
const goals = await forge.db.queryGoals({ ... });
```

### 5.3 Search API (`forge.search`)

**Requiere:** `db_read`

```javascript
// Busqueda full-text
const results = await forge.search.fts({
  query: 'meeting notes',
  entity_types: ['notes', 'tasks'],
  limit: 10
});

// Busqueda semantica
const similar = await forge.search.semantic({
  query: 'productivity tips',
  entity_types: ['notes'],
  limit: 5
});
```

### 5.4 Settings API (`forge.settings`)

**No requiere permisos especiales** (solo settings del plugin)

```javascript
// Leer setting del plugin
const value = await forge.settings.get('my-plugin.option1');

// Escribir setting del plugin
await forge.settings.set('my-plugin.option1', 'new-value');

// Leer setting global (solo lectura)
const theme = await forge.settings.getGlobal('ui.theme');
```

### 5.5 UI API (`forge.ui`)

```javascript
// Mostrar notificacion
forge.ui.toast({
  level: 'info',
  title: 'Success',
  message: 'Task created',
  duration_ms: 3000
});

// Mostrar dialogo de confirmacion
const confirmed = await forge.ui.confirm({
  title: 'Confirm Action',
  message: 'Are you sure?'
});

// Abrir panel del plugin
forge.ui.openPanel('my-panel');
```

### 5.6 Logging API (`forge.log`)

```javascript
forge.log.info('Processing started');
forge.log.warn('Something unusual');
forge.log.error('Failed to process', { error: err.message });
// Logs van a logs/plugins-YYYYMMDD.jsonl
```

---

## 6) Commands

Los commands son funciones que el usuario o Forge pueden invocar.

### 6.1 Registrar Command

En `main.js`:

```javascript
forge.commands.register('my-plugin.doSomething', async (input) => {
  // Validar input contra schema (automatico si se define schema)
  const { taskIds } = input;

  // Hacer trabajo
  const results = [];
  for (const id of taskIds) {
    const task = await forge.db.getTask(id);
    // procesar...
    results.push({ id, processed: true });
  }

  // Retornar output (validado contra schema)
  return { results };
});
```

### 6.2 Input/Output Schemas

`schemas/input.json`:
```json
{
  "type": "object",
  "required": ["taskIds"],
  "properties": {
    "taskIds": {
      "type": "array",
      "items": { "type": "string" },
      "minItems": 1
    }
  }
}
```

`schemas/output.json`:
```json
{
  "type": "object",
  "required": ["results"],
  "properties": {
    "results": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "processed": { "type": "boolean" }
        }
      }
    }
  }
}
```

---

## 7) Hooks

### 7.1 Lifecycle Hooks

```javascript
// main.js

export function onInstall() {
  // Ejecutado una vez al instalar
  forge.log.info('Plugin installed');
}

export function onEnable() {
  // Ejecutado cada vez que se habilita
  forge.log.info('Plugin enabled');
}

export function onDisable() {
  // Ejecutado al deshabilitar
  forge.log.info('Plugin disabled');
}

export function onUninstall() {
  // Ejecutado antes de desinstalar
  // Limpiar datos del plugin si es necesario
  forge.log.info('Plugin uninstalled');
}
```

---

## 8) UI Panels

### 8.1 Definir Panel

En `manifest.json`:
```json
{
  "ui": {
    "panels": [
      {
        "id": "my-panel",
        "title": "My Panel",
        "location": "sidebar",
        "component": "MyPanel"
      }
    ]
  }
}
```

### 8.2 Implementar Panel

El sistema de UI de plugins usa un subset de HTML/CSS renderizado de forma segura:

```javascript
// En main.js
forge.ui.registerPanel('MyPanel', {
  render: () => `
    <div class="my-panel">
      <h3>My Panel</h3>
      <button onclick="handleClick()">Click Me</button>
    </div>
  `,

  styles: `
    .my-panel { padding: 16px; }
    .my-panel h3 { margin: 0 0 16px; }
  `,

  handlers: {
    handleClick: () => {
      forge.ui.toast({ level: 'info', title: 'Clicked!' });
    }
  }
});
```

---

## 9) Firma del Plugin

### 9.1 Generar Keypair

```bash
# Generar keypair Ed25519 (una vez)
openssl genpkey -algorithm Ed25519 -out plugin-key.pem
openssl pkey -in plugin-key.pem -pubout -out plugin-key.pub
```

### 9.2 Firmar Plugin

```bash
# Calcular hash del contenido (excluyendo signature)
sha256sum manifest.json main.js schemas/* assets/* > content-hash.txt

# Firmar el hash
openssl pkeyutl -sign -inkey plugin-key.pem \
  -in content-hash.txt -out signature.bin

# Convertir a base64 para manifest
base64 signature.bin > signature.b64
base64 plugin-key.pub > pubkey.b64
```

### 9.3 Agregar Firma al Manifest

```json
{
  "signature": {
    "algo": "ed25519",
    "public_key": "<contenido de pubkey.b64>",
    "signed_hash": "<contenido de signature.b64>"
  }
}
```

---

## 10) Empaquetar Plugin

### 10.1 Estructura del ZIP

```bash
# Crear archivo ZIP
zip -r my-plugin-1.0.0.zip \
  manifest.json \
  main.js \
  schemas/ \
  assets/ \
  README.md
```

### 10.2 Instalar Plugin

En Forge:
1. Settings > Plugins > Install
2. Seleccionar archivo `.zip`
3. Revisar y aprobar permisos
4. Enable plugin

---

## 11) Debugging

### 11.1 Modo Desarrollo

Para desarrollo local sin firma:

```bash
# En Forge settings
security.allow_unsigned_plugins = true  # SOLO para dev!
```

### 11.2 Logs del Plugin

```bash
# Ver logs del plugin
tail -f ~/.forge/logs/plugins-$(date +%Y%m%d).jsonl | jq
```

### 11.3 Errores Comunes

| Error | Causa | Solucion |
|-------|-------|----------|
| `PluginPermissionError` | Falta permiso | Agregar a manifest |
| `PluginSignatureError` | Firma invalida | Re-firmar plugin |
| `PluginSchemaError` | Input/output no valida | Revisar schemas |
| `PluginRuntimeError` | Error en codigo JS | Ver logs para stack trace |

---

## 12) Best Practices

### 12.1 Seguridad

- Solicitar solo permisos necesarios
- Validar todos los inputs
- No almacenar datos sensibles en plugin state
- Usar `network_external` solo si es imprescindible

### 12.2 Performance

- Evitar queries en loops (usar batch)
- Cachear datos cuando sea posible
- No bloquear UI con operaciones largas

### 12.3 UX

- Mostrar feedback claro al usuario
- Manejar errores gracefully
- Documentar funcionalidad en README

---

## 13) Ejemplo Completo

### 13.1 Plugin "Task Counter"

`manifest.json`:
```json
{
  "plugin_id": "com.forge.task-counter",
  "name": "Task Counter",
  "version": "1.0.0",
  "description": "Shows task counts by status",
  "author": { "name": "Forge Team" },
  "license": "MIT",
  "min_forge_version": "1.0.0",
  "permissions": ["db_read"],
  "commands": [
    {
      "name": "task-counter.count",
      "description": "Count tasks by status"
    }
  ],
  "ui": {
    "panels": [
      {
        "id": "counter-panel",
        "title": "Task Counts",
        "location": "sidebar",
        "component": "CounterPanel"
      }
    ]
  },
  "signature": { ... }
}
```

`main.js`:
```javascript
// Command: count tasks
forge.commands.register('task-counter.count', async () => {
  const statuses = ['inbox', 'next', 'waiting', 'someday', 'done'];
  const counts = {};

  for (const status of statuses) {
    const tasks = await forge.db.queryTasks({ status: [status] });
    counts[status] = tasks.total;
  }

  return { counts };
});

// UI Panel
forge.ui.registerPanel('CounterPanel', {
  state: { counts: null, loading: true },

  async onMount() {
    const result = await forge.commands.invoke('task-counter.count');
    this.state.counts = result.counts;
    this.state.loading = false;
    this.refresh();
  },

  render() {
    if (this.state.loading) {
      return '<div class="loading">Loading...</div>';
    }

    const { counts } = this.state;
    return `
      <div class="counter-panel">
        <h3>Task Counts</h3>
        <ul>
          ${Object.entries(counts).map(([status, count]) =>
            `<li><strong>${status}:</strong> ${count}</li>`
          ).join('')}
        </ul>
      </div>
    `;
  },

  styles: `
    .counter-panel { padding: 16px; }
    .counter-panel ul { list-style: none; padding: 0; }
    .counter-panel li { margin: 8px 0; }
  `
});

export function onEnable() {
  forge.log.info('Task Counter enabled');
}
```

---

## Referencias

- [API IPC Reference](api-ipc.md)
- [JSON Schema Spec](https://json-schema.org/)
- [Ed25519 Signing](https://en.wikipedia.org/wiki/EdDSA)
