/**
 * Tauri IPC API Client
 *
 * Provides typed access to all backend commands via Tauri's invoke API.
 */

import type {
	// Tasks
	CreateTaskRequest,
	TaskResponse,
	UpdateTaskRequest,
	ListTasksRequest,
	ListTasksResponse,
	// Notes
	CreateNoteRequest,
	NoteResponse,
	UpdateNoteRequest,
	ListNotesRequest,
	ListNotesResponse,
	// Goals
	CreateGoalRequest,
	GoalResponse,
	UpdateGoalRequest,
	ListGoalsRequest,
	ListGoalsResponse,
	// Search
	FtsSearchRequest,
	FtsSearchResponse,
	SemanticSearchRequest,
	SemanticSearchResponse,
	// Settings
	GetSettingRequest,
	GetSettingResponse,
	SetSettingRequest,
	GetAllSettingsResponse,
	// Delete
	DeleteRequest,
	DeleteResponse
} from './types';

// Check if Tauri is available
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

/**
 * Invoke a Tauri command
 */
async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
	if (!isTauri) {
		// Mock response for development outside Tauri
		console.warn(`Tauri not available. Mock call to ${command}`, args);
		throw new Error('Tauri is not available');
	}

	const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');

	try {
		const result = await tauriInvoke<{ ok: boolean; data?: T; error?: { code: string; message: string } }>(
			command,
			args
		);

		if (result.ok && result.data !== undefined) {
			return result.data;
		}

		if (result.error) {
			throw new Error(`${result.error.code}: ${result.error.message}`);
		}

		throw new Error('Unknown error');
	} catch (e) {
		console.error(`IPC error for ${command}:`, e);
		throw e;
	}
}

/**
 * API client with all available commands
 */
export const api = {
	// =========================================================================
	// Tasks
	// =========================================================================
	tasks: {
		create: (data: CreateTaskRequest): Promise<TaskResponse> =>
			invoke('v1_tasks_create', { request: data }),

		get: (id: string): Promise<TaskResponse> =>
			invoke('v1_tasks_get', { request: { id } }),

		update: (data: UpdateTaskRequest): Promise<void> =>
			invoke('v1_tasks_update', { request: data }),

		delete: (data: DeleteRequest): Promise<DeleteResponse> =>
			invoke('v1_tasks_delete', { request: data }),

		list: (filter: ListTasksRequest = {}): Promise<ListTasksResponse> =>
			invoke('v1_tasks_list', { request: filter })
	},

	// =========================================================================
	// Notes
	// =========================================================================
	notes: {
		create: (data: CreateNoteRequest): Promise<NoteResponse> =>
			invoke('v1_notes_create', { request: data }),

		get: (id: string): Promise<NoteResponse> =>
			invoke('v1_notes_get', { request: { id } }),

		update: (data: UpdateNoteRequest): Promise<void> =>
			invoke('v1_notes_update', { request: data }),

		delete: (data: DeleteRequest): Promise<DeleteResponse> =>
			invoke('v1_notes_delete', { request: data }),

		list: (filter: ListNotesRequest = {}): Promise<ListNotesResponse> =>
			invoke('v1_notes_list', { request: filter })
	},

	// =========================================================================
	// Goals
	// =========================================================================
	goals: {
		create: (data: CreateGoalRequest): Promise<GoalResponse> =>
			invoke('v1_goals_create', { request: data }),

		get: (id: string): Promise<GoalResponse> =>
			invoke('v1_goals_get', { request: { id } }),

		update: (data: UpdateGoalRequest): Promise<void> =>
			invoke('v1_goals_update', { request: data }),

		delete: (data: DeleteRequest): Promise<DeleteResponse> =>
			invoke('v1_goals_delete', { request: data }),

		list: (filter: ListGoalsRequest = {}): Promise<ListGoalsResponse> =>
			invoke('v1_goals_list', { request: filter })
	},

	// =========================================================================
	// Search
	// =========================================================================
	search: {
		fts: (request: FtsSearchRequest): Promise<FtsSearchResponse> =>
			invoke('v1_search_fts', { request }),

		semantic: (request: SemanticSearchRequest): Promise<SemanticSearchResponse> =>
			invoke('v1_search_semantic', { request })
	},

	// =========================================================================
	// Settings
	// =========================================================================
	settings: {
		get: (key: string): Promise<GetSettingResponse> =>
			invoke('v1_settings_get', { request: { key } }),

		set: (key: string, value: unknown): Promise<void> =>
			invoke('v1_settings_set', { request: { key, value } }),

		getAll: (): Promise<GetAllSettingsResponse> =>
			invoke('v1_settings_get_all', {})
	}
};
