/**
 * TypeScript types for IPC API
 *
 * These types mirror the Rust DTOs in forge-api-ipc
 */

export type EntityId = string;
export type TimestampMs = number;

// =============================================================================
// Task Types
// =============================================================================

export interface CreateTaskRequest {
	title: string;
	description_md?: string;
	status?: string;
	priority?: number;
	due_at?: TimestampMs;
	scheduled_at?: TimestampMs;
	parent_task_id?: EntityId;
	goal_id?: EntityId;
	context_tags?: string[];
	energy_level?: string;
	time_estimate_min?: number;
	recurrence_rule?: string;
}

export interface TaskResponse {
	id: EntityId;
	title: string;
	description_md?: string;
	status: string;
	priority?: number;
	priority_score?: number;
	due_at?: TimestampMs;
	scheduled_at?: TimestampMs;
	completed_at?: TimestampMs;
	parent_task_id?: EntityId;
	goal_id?: EntityId;
	context_tags: string[];
	energy_level?: string;
	time_estimate_min?: number;
	recurrence_rule?: string;
	created_at: TimestampMs;
	updated_at: TimestampMs;
}

export interface UpdateTaskRequest {
	id: EntityId;
	title?: string;
	description_md?: string;
	status?: string;
	priority?: number;
	due_at?: TimestampMs;
	scheduled_at?: TimestampMs;
	goal_id?: EntityId;
	context_tags?: string[];
	energy_level?: string;
	time_estimate_min?: number;
}

export interface ListTasksRequest {
	status?: string[];
	goal_id?: EntityId;
	due_before?: TimestampMs;
	due_after?: TimestampMs;
	context_tags?: string[];
	include_deleted?: boolean;
	order_by?: string;
	order_dir?: string;
	limit?: number;
	offset?: number;
}

export interface ListTasksResponse {
	tasks: TaskResponse[];
	total: number;
	has_more: boolean;
}

// =============================================================================
// Note Types
// =============================================================================

export interface CreateNoteRequest {
	title: string;
	content_md: string;
	note_type?: string;
	tags?: string[];
	pinned?: boolean;
}

export interface NoteResponse {
	id: EntityId;
	title: string;
	content_md: string;
	note_type: string;
	tags: string[];
	pinned: boolean;
	created_at: TimestampMs;
	updated_at: TimestampMs;
}

export interface UpdateNoteRequest {
	id: EntityId;
	title?: string;
	content_md?: string;
	note_type?: string;
	tags?: string[];
	pinned?: boolean;
}

export interface ListNotesRequest {
	note_type?: string[];
	tags?: string[];
	pinned_only?: boolean;
	include_deleted?: boolean;
	order_by?: string;
	order_dir?: string;
	limit?: number;
	offset?: number;
}

export interface ListNotesResponse {
	notes: NoteResponse[];
	total: number;
	has_more: boolean;
}

// =============================================================================
// Goal Types
// =============================================================================

export interface CreateGoalRequest {
	title: string;
	description_md?: string;
	status?: string;
	horizon?: string;
	target_date?: TimestampMs;
	parent_goal_id?: EntityId;
}

export interface GoalResponse {
	id: EntityId;
	title: string;
	description_md?: string;
	status: string;
	horizon: string;
	target_date?: TimestampMs;
	progress_percent: number;
	parent_goal_id?: EntityId;
	created_at: TimestampMs;
	updated_at: TimestampMs;
}

export interface UpdateGoalRequest {
	id: EntityId;
	title?: string;
	description_md?: string;
	status?: string;
	horizon?: string;
	target_date?: TimestampMs;
	progress_percent?: number;
}

export interface ListGoalsRequest {
	status?: string[];
	horizon?: string[];
	include_deleted?: boolean;
	limit?: number;
	offset?: number;
}

export interface ListGoalsResponse {
	goals: GoalResponse[];
	total: number;
	has_more: boolean;
}

// =============================================================================
// Search Types
// =============================================================================

export interface FtsSearchRequest {
	query: string;
	entity_types?: string[];
	limit?: number;
}

export interface SearchResult {
	entity_type: string;
	entity_id: EntityId;
	title: string;
	snippet?: string;
	score: number;
}

export interface FtsSearchResponse {
	results: SearchResult[];
	total: number;
	query_time_ms: number;
}

export interface SemanticSearchRequest {
	query: string;
	entity_types?: string[];
	limit?: number;
	min_score?: number;
}

export interface SemanticSearchResult {
	entity_type: string;
	entity_id: EntityId;
	title: string;
	similarity: number;
}

export interface SemanticSearchResponse {
	results: SemanticSearchResult[];
	query_time_ms: number;
	embedding_model: string;
}

// =============================================================================
// Settings Types
// =============================================================================

export interface GetSettingRequest {
	key: string;
}

export interface GetSettingResponse {
	key: string;
	value: unknown;
}

export interface SetSettingRequest {
	key: string;
	value: unknown;
}

export interface GetAllSettingsResponse {
	settings: Record<string, unknown>;
}

// =============================================================================
// Common Types
// =============================================================================

export interface DeleteRequest {
	id: EntityId;
	hard_delete?: boolean;
}

export interface DeleteResponse {
	id: EntityId;
	deleted_at: TimestampMs;
}

export interface ApiError {
	code: string;
	message: string;
	details?: Record<string, unknown>;
}
