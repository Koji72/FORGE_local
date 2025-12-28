import { writable, derived } from 'svelte/store';
import { api } from '$api/client';
import type { TaskResponse, ListTasksRequest } from '$api/types';

interface TaskState {
	items: TaskResponse[];
	loading: boolean;
	error: string | null;
	filter: ListTasksRequest;
}

function createTaskStore() {
	const initialState: TaskState = {
		items: [],
		loading: false,
		error: null,
		filter: {}
	};

	const { subscribe, set, update } = writable<TaskState>(initialState);

	return {
		subscribe,

		async load(filter?: ListTasksRequest) {
			update((s) => ({ ...s, loading: true, error: null }));

			try {
				const response = await api.tasks.list(filter || {});
				update((s) => ({
					...s,
					items: response.tasks,
					loading: false,
					filter: filter || {}
				}));
			} catch (e) {
				update((s) => ({
					...s,
					loading: false,
					error: e instanceof Error ? e.message : 'Failed to load tasks'
				}));
			}
		},

		async create(data: { title: string; description_md?: string; status?: string }) {
			try {
				const task = await api.tasks.create(data);
				update((s) => ({ ...s, items: [task, ...s.items] }));
				return task;
			} catch (e) {
				throw e;
			}
		},

		async update(id: string, data: Partial<TaskResponse>) {
			try {
				await api.tasks.update({ id, ...data });
				update((s) => ({
					...s,
					items: s.items.map((t) => (t.id === id ? { ...t, ...data } : t))
				}));
			} catch (e) {
				throw e;
			}
		},

		async delete(id: string) {
			try {
				await api.tasks.delete({ id });
				update((s) => ({
					...s,
					items: s.items.filter((t) => t.id !== id)
				}));
			} catch (e) {
				throw e;
			}
		},

		async toggleComplete(id: string) {
			const currentTasks = await new Promise<TaskResponse[]>((resolve) => {
				const unsubscribe = subscribe((s) => {
					resolve(s.items);
					unsubscribe();
				});
			});

			const task = currentTasks.find((t) => t.id === id);
			if (!task) return;

			const newStatus = task.status === 'done' ? 'inbox' : 'done';
			await this.update(id, { status: newStatus });
		}
	};
}

export const taskStore = createTaskStore();

// Derived store for just the tasks array (convenience)
export const tasks = derived(taskStore, ($store) => $store.items);
