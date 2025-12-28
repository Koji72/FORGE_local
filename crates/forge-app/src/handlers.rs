//! Tauri command handlers
//!
//! All IPC command handlers for the Forge application.

use crate::state::{ApiResponse, AppError, SharedState, new_entity_id, now_ms};
use forge_api_ipc::commands::*;
use tauri::State;

// =============================================================================
// Task Handlers
// =============================================================================

#[tauri::command]
pub async fn v1_tasks_create(
    state: State<'_, SharedState>,
    request: CreateTaskRequest,
) -> Result<ApiResponse<TaskResponse>, ()> {
    let state = state.read().await;
    let id = new_entity_id();
    let now = now_ms();

    let task = forge_storage::Task {
        id: id.clone(),
        title: request.title,
        description_md: request.description_md,
        status: request.status.unwrap_or_else(|| "inbox".to_string()),
        priority: request.priority,
        priority_score: None,
        due_at: request.due_at,
        scheduled_at: request.scheduled_at,
        completed_at: None,
        parent_task_id: request.parent_task_id,
        goal_id: request.goal_id,
        context_tags: request.context_tags,
        energy_level: request.energy_level,
        time_estimate_min: request.time_estimate_min,
        recurrence_rule: request.recurrence_rule,
        created_at: now,
        updated_at: now,
        deleted_at: None,
    };

    match state.tasks().create(&task).await {
        Ok(_) => Ok(ApiResponse::success(task_to_response(task))),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_tasks_get(
    state: State<'_, SharedState>,
    request: GetByIdRequest,
) -> Result<ApiResponse<TaskResponse>, ()> {
    let state = state.read().await;

    match state.tasks().get(&request.id).await {
        Ok(Some(task)) => Ok(ApiResponse::success(task_to_response(task))),
        Ok(None) => Ok(ApiResponse::error(AppError::NotFound(format!("Task {} not found", request.id)))),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_tasks_update(
    state: State<'_, SharedState>,
    request: UpdateTaskRequest,
) -> Result<ApiResponse<UpdateResponse>, ()> {
    let state = state.read().await;
    let now = now_ms();

    // Get existing task
    let task = match state.tasks().get(&request.id).await {
        Ok(Some(t)) => t,
        Ok(None) => return Ok(ApiResponse::error(AppError::NotFound(format!("Task {} not found", request.id)))),
        Err(e) => return Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    };

    let mut changes = Vec::new();

    let updated_task = forge_storage::Task {
        id: task.id,
        title: request.title.unwrap_or_else(|| { task.title.clone() }),
        description_md: request.description_md.or(task.description_md),
        status: request.status.unwrap_or_else(|| {
            if request.status.is_some() { changes.push("status".to_string()); }
            task.status.clone()
        }),
        priority: request.priority.or(task.priority),
        priority_score: task.priority_score,
        due_at: request.due_at.or(task.due_at),
        scheduled_at: request.scheduled_at.or(task.scheduled_at),
        completed_at: task.completed_at,
        parent_task_id: task.parent_task_id,
        goal_id: request.goal_id.or(task.goal_id),
        context_tags: request.context_tags.unwrap_or(task.context_tags),
        energy_level: request.energy_level.or(task.energy_level),
        time_estimate_min: request.time_estimate_min.or(task.time_estimate_min),
        recurrence_rule: task.recurrence_rule,
        created_at: task.created_at,
        updated_at: now,
        deleted_at: task.deleted_at,
    };

    match state.tasks().update(&updated_task).await {
        Ok(_) => Ok(ApiResponse::success(UpdateResponse {
            id: request.id,
            updated_at: now,
            changes,
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_tasks_delete(
    state: State<'_, SharedState>,
    request: DeleteRequest,
) -> Result<ApiResponse<DeleteResponse>, ()> {
    let state = state.read().await;
    let now = now_ms();

    match state.tasks().delete(&request.id, now).await {
        Ok(_) => Ok(ApiResponse::success(DeleteResponse {
            id: request.id,
            deleted_at: now,
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_tasks_list(
    state: State<'_, SharedState>,
    request: ListTasksRequest,
) -> Result<ApiResponse<ListTasksResponse>, ()> {
    let state = state.read().await;

    let filter = forge_storage::TaskFilter {
        status: request.status,
        goal_id: request.goal_id,
        due_before: request.due_before,
        due_after: request.due_after,
        context_tags: request.context_tags,
        include_deleted: request.include_deleted,
        order_by: request.order_by,
        order_dir: request.order_dir,
        limit: request.limit,
        offset: request.offset,
    };

    match state.tasks().list(filter.clone()).await {
        Ok(tasks) => {
            let total = state.tasks().count(filter).await.unwrap_or(0);
            let limit = request.limit.unwrap_or(50) as u64;
            let offset = request.offset.unwrap_or(0) as u64;

            Ok(ApiResponse::success(ListTasksResponse {
                tasks: tasks.into_iter().map(task_to_response).collect(),
                total,
                has_more: offset + limit < total,
            }))
        }
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

// =============================================================================
// Note Handlers
// =============================================================================

#[tauri::command]
pub async fn v1_notes_create(
    state: State<'_, SharedState>,
    request: CreateNoteRequest,
) -> Result<ApiResponse<NoteResponse>, ()> {
    let state = state.read().await;
    let id = new_entity_id();
    let now = now_ms();

    let note = forge_storage::Note {
        id: id.clone(),
        title: request.title,
        content_md: request.content_md,
        note_type: request.note_type.unwrap_or_else(|| "fleeting".to_string()),
        tags: request.tags,
        pinned: request.pinned,
        created_at: now,
        updated_at: now,
        deleted_at: None,
    };

    match state.notes().create(&note).await {
        Ok(_) => Ok(ApiResponse::success(note_to_response(note))),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_notes_get(
    state: State<'_, SharedState>,
    request: GetByIdRequest,
) -> Result<ApiResponse<NoteResponse>, ()> {
    let state = state.read().await;

    match state.notes().get(&request.id).await {
        Ok(Some(note)) => Ok(ApiResponse::success(note_to_response(note))),
        Ok(None) => Ok(ApiResponse::error(AppError::NotFound(format!("Note {} not found", request.id)))),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_notes_update(
    state: State<'_, SharedState>,
    request: UpdateNoteRequest,
) -> Result<ApiResponse<UpdateResponse>, ()> {
    let state = state.read().await;
    let now = now_ms();

    let note = match state.notes().get(&request.id).await {
        Ok(Some(n)) => n,
        Ok(None) => return Ok(ApiResponse::error(AppError::NotFound(format!("Note {} not found", request.id)))),
        Err(e) => return Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    };

    let updated_note = forge_storage::Note {
        id: note.id,
        title: request.title.unwrap_or(note.title),
        content_md: request.content_md.unwrap_or(note.content_md),
        note_type: request.note_type.unwrap_or(note.note_type),
        tags: request.tags.unwrap_or(note.tags),
        pinned: request.pinned.unwrap_or(note.pinned),
        created_at: note.created_at,
        updated_at: now,
        deleted_at: note.deleted_at,
    };

    match state.notes().update(&updated_note).await {
        Ok(_) => Ok(ApiResponse::success(UpdateResponse {
            id: request.id,
            updated_at: now,
            changes: vec![],
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_notes_delete(
    state: State<'_, SharedState>,
    request: DeleteRequest,
) -> Result<ApiResponse<DeleteResponse>, ()> {
    let state = state.read().await;
    let now = now_ms();

    match state.notes().delete(&request.id, now).await {
        Ok(_) => Ok(ApiResponse::success(DeleteResponse {
            id: request.id,
            deleted_at: now,
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_notes_list(
    state: State<'_, SharedState>,
    request: ListNotesRequest,
) -> Result<ApiResponse<ListNotesResponse>, ()> {
    let state = state.read().await;

    let filter = forge_storage::NoteFilter {
        note_type: request.note_type,
        tags: request.tags,
        pinned_only: request.pinned_only,
        include_deleted: request.include_deleted,
        order_by: request.order_by,
        order_dir: request.order_dir,
        limit: request.limit,
        offset: request.offset,
    };

    match state.notes().list(filter.clone()).await {
        Ok(notes) => {
            let total = state.notes().count(filter).await.unwrap_or(0);
            let limit = request.limit.unwrap_or(50) as u64;
            let offset = request.offset.unwrap_or(0) as u64;

            Ok(ApiResponse::success(ListNotesResponse {
                notes: notes.into_iter().map(note_to_response).collect(),
                total,
                has_more: offset + limit < total,
            }))
        }
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

// =============================================================================
// Goal Handlers
// =============================================================================

#[tauri::command]
pub async fn v1_goals_create(
    state: State<'_, SharedState>,
    request: CreateGoalRequest,
) -> Result<ApiResponse<GoalResponse>, ()> {
    let state = state.read().await;
    let id = new_entity_id();
    let now = now_ms();

    let goal = forge_storage::Goal {
        id: id.clone(),
        title: request.title,
        description_md: request.description_md,
        status: request.status.unwrap_or_else(|| "active".to_string()),
        horizon: request.horizon.unwrap_or_else(|| "year".to_string()),
        target_date: request.target_date,
        progress_percent: 0,
        parent_goal_id: request.parent_goal_id,
        created_at: now,
        updated_at: now,
        deleted_at: None,
    };

    match state.goals().create(&goal).await {
        Ok(_) => Ok(ApiResponse::success(goal_to_response(goal))),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_goals_get(
    state: State<'_, SharedState>,
    request: GetByIdRequest,
) -> Result<ApiResponse<GoalResponse>, ()> {
    let state = state.read().await;

    match state.goals().get(&request.id).await {
        Ok(Some(goal)) => Ok(ApiResponse::success(goal_to_response(goal))),
        Ok(None) => Ok(ApiResponse::error(AppError::NotFound(format!("Goal {} not found", request.id)))),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_goals_update(
    state: State<'_, SharedState>,
    request: UpdateGoalRequest,
) -> Result<ApiResponse<UpdateResponse>, ()> {
    let state = state.read().await;
    let now = now_ms();

    let goal = match state.goals().get(&request.id).await {
        Ok(Some(g)) => g,
        Ok(None) => return Ok(ApiResponse::error(AppError::NotFound(format!("Goal {} not found", request.id)))),
        Err(e) => return Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    };

    let updated_goal = forge_storage::Goal {
        id: goal.id,
        title: request.title.unwrap_or(goal.title),
        description_md: request.description_md.or(goal.description_md),
        status: request.status.unwrap_or(goal.status),
        horizon: request.horizon.unwrap_or(goal.horizon),
        target_date: request.target_date.or(goal.target_date),
        progress_percent: request.progress_percent.unwrap_or(goal.progress_percent),
        parent_goal_id: goal.parent_goal_id,
        created_at: goal.created_at,
        updated_at: now,
        deleted_at: goal.deleted_at,
    };

    match state.goals().update(&updated_goal).await {
        Ok(_) => Ok(ApiResponse::success(UpdateResponse {
            id: request.id,
            updated_at: now,
            changes: vec![],
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_goals_delete(
    state: State<'_, SharedState>,
    request: DeleteRequest,
) -> Result<ApiResponse<DeleteResponse>, ()> {
    let state = state.read().await;
    let now = now_ms();

    match state.goals().delete(&request.id, now).await {
        Ok(_) => Ok(ApiResponse::success(DeleteResponse {
            id: request.id,
            deleted_at: now,
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_goals_list(
    state: State<'_, SharedState>,
    request: ListGoalsRequest,
) -> Result<ApiResponse<ListGoalsResponse>, ()> {
    let state = state.read().await;

    let filter = forge_storage::GoalFilter {
        status: request.status,
        horizon: request.horizon,
        include_deleted: request.include_deleted,
        limit: request.limit,
        offset: request.offset,
    };

    match state.goals().list(filter.clone()).await {
        Ok(goals) => {
            let total = state.goals().count(filter).await.unwrap_or(0);
            let limit = request.limit.unwrap_or(50) as u64;
            let offset = request.offset.unwrap_or(0) as u64;

            Ok(ApiResponse::success(ListGoalsResponse {
                goals: goals.into_iter().map(goal_to_response).collect(),
                total,
                has_more: offset + limit < total,
            }))
        }
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

// =============================================================================
// Search Handlers
// =============================================================================

#[tauri::command]
pub async fn v1_search_fts(
    state: State<'_, SharedState>,
    request: FtsSearchRequest,
) -> Result<ApiResponse<FtsSearchResponse>, ()> {
    let _state = state.read().await;
    let start = std::time::Instant::now();

    // TODO: Implement FTS search using FTS5 tables
    // For now, return empty results

    Ok(ApiResponse::success(FtsSearchResponse {
        results: vec![],
        total: 0,
        query_time_ms: start.elapsed().as_millis() as u64,
    }))
}

#[tauri::command]
pub async fn v1_search_semantic(
    state: State<'_, SharedState>,
    request: SemanticSearchRequest,
) -> Result<ApiResponse<SemanticSearchResponse>, ()> {
    let _state = state.read().await;
    let start = std::time::Instant::now();

    // TODO: Implement semantic search using embeddings
    // For now, return empty results

    Ok(ApiResponse::success(SemanticSearchResponse {
        results: vec![],
        query_time_ms: start.elapsed().as_millis() as u64,
        embedding_model: "none".to_string(),
    }))
}

// =============================================================================
// Settings Handlers
// =============================================================================

#[tauri::command]
pub async fn v1_settings_get(
    state: State<'_, SharedState>,
    request: GetSettingRequest,
) -> Result<ApiResponse<GetSettingResponse>, ()> {
    let state = state.read().await;

    match state.settings().get(&request.key).await {
        Ok(Some(value)) => Ok(ApiResponse::success(GetSettingResponse {
            key: request.key,
            value,
        })),
        Ok(None) => Ok(ApiResponse::success(GetSettingResponse {
            key: request.key,
            value: serde_json::Value::Null,
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_settings_set(
    state: State<'_, SharedState>,
    request: SetSettingRequest,
) -> Result<ApiResponse<SetSettingResponse>, ()> {
    let state = state.read().await;

    match state.settings().set(&request.key, &request.value).await {
        Ok(_) => Ok(ApiResponse::success(SetSettingResponse {
            key: request.key,
            updated: true,
        })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

#[tauri::command]
pub async fn v1_settings_get_all(
    state: State<'_, SharedState>,
) -> Result<ApiResponse<GetAllSettingsResponse>, ()> {
    let state = state.read().await;

    match state.settings().get_all().await {
        Ok(settings) => Ok(ApiResponse::success(GetAllSettingsResponse { settings })),
        Err(e) => Ok(ApiResponse::error(AppError::Database(e.to_string()))),
    }
}

// =============================================================================
// Vault Handlers
// =============================================================================

#[tauri::command]
pub async fn v1_vault_info(
    state: State<'_, SharedState>,
) -> Result<ApiResponse<VaultInfo>, ()> {
    let state = state.read().await;

    // Get entity counts
    let task_count = state.tasks().count(forge_storage::TaskFilter::default()).await.unwrap_or(0);
    let note_count = state.notes().count(forge_storage::NoteFilter::default()).await.unwrap_or(0);
    let goal_count = state.goals().count(forge_storage::GoalFilter::default()).await.unwrap_or(0);

    // Get file size
    let db_path = state.vault_path.join("forge.db");
    let size_bytes = std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);

    Ok(ApiResponse::success(VaultInfo {
        vault_id: state.vault_id.clone(),
        db_path: db_path.to_string_lossy().to_string(),
        encrypted: false, // TODO: Check if SQLCipher is enabled
        created_at: 0, // TODO: Get from schema_version table
        size_bytes,
        entity_counts: EntityCounts {
            tasks: task_count,
            notes: note_count,
            goals: goal_count,
        },
    }))
}

// =============================================================================
// Helper Functions
// =============================================================================

fn task_to_response(task: forge_storage::Task) -> TaskResponse {
    TaskResponse {
        id: task.id,
        title: task.title,
        description_md: task.description_md,
        status: task.status,
        priority: task.priority,
        priority_score: task.priority_score,
        due_at: task.due_at,
        scheduled_at: task.scheduled_at,
        completed_at: task.completed_at,
        parent_task_id: task.parent_task_id,
        goal_id: task.goal_id,
        context_tags: task.context_tags,
        energy_level: task.energy_level,
        time_estimate_min: task.time_estimate_min,
        recurrence_rule: task.recurrence_rule,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }
}

fn note_to_response(note: forge_storage::Note) -> NoteResponse {
    NoteResponse {
        id: note.id,
        title: note.title,
        content_md: note.content_md,
        note_type: note.note_type,
        tags: note.tags,
        pinned: note.pinned,
        created_at: note.created_at,
        updated_at: note.updated_at,
    }
}

fn goal_to_response(goal: forge_storage::Goal) -> GoalResponse {
    GoalResponse {
        id: goal.id,
        title: goal.title,
        description_md: goal.description_md,
        status: goal.status,
        horizon: goal.horizon,
        target_date: goal.target_date,
        progress_percent: goal.progress_percent,
        parent_goal_id: goal.parent_goal_id,
        created_at: goal.created_at,
        updated_at: goal.updated_at,
    }
}
