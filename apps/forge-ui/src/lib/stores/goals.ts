import { writable, derived } from 'svelte/store';
import { api } from '$api/client';
import type { GoalResponse, ListGoalsRequest } from '$api/types';

interface GoalState {
	items: GoalResponse[];
	loading: boolean;
	error: string | null;
	filter: ListGoalsRequest;
}

function createGoalStore() {
	const initialState: GoalState = {
		items: [],
		loading: false,
		error: null,
		filter: {}
	};

	const { subscribe, set, update } = writable<GoalState>(initialState);

	return {
		subscribe,

		async load(filter?: ListGoalsRequest) {
			update((s) => ({ ...s, loading: true, error: null }));

			try {
				const response = await api.goals.list(filter || {});
				update((s) => ({
					...s,
					items: response.goals,
					loading: false,
					filter: filter || {}
				}));
			} catch (e) {
				update((s) => ({
					...s,
					loading: false,
					error: e instanceof Error ? e.message : 'Failed to load goals'
				}));
			}
		},

		async create(data: { title: string; description_md?: string; horizon?: string }) {
			try {
				const goal = await api.goals.create(data);
				update((s) => ({ ...s, items: [goal, ...s.items] }));
				return goal;
			} catch (e) {
				throw e;
			}
		},

		async update(id: string, data: Partial<GoalResponse>) {
			try {
				await api.goals.update({ id, ...data });
				update((s) => ({
					...s,
					items: s.items.map((g) => (g.id === id ? { ...g, ...data } : g))
				}));
			} catch (e) {
				throw e;
			}
		},

		async delete(id: string) {
			try {
				await api.goals.delete({ id });
				update((s) => ({
					...s,
					items: s.items.filter((g) => g.id !== id)
				}));
			} catch (e) {
				throw e;
			}
		},

		async updateProgress(id: string, progress: number) {
			await this.update(id, { progress_percent: Math.min(100, Math.max(0, progress)) });
		}
	};
}

export const goalStore = createGoalStore();

// Derived store for just the goals array
export const goals = derived(goalStore, ($store) => $store.items);
