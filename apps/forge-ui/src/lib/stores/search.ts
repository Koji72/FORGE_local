import { writable } from 'svelte/store';
import { api } from '$api/client';
import type { SearchResult, SemanticSearchResult } from '$api/types';

interface SearchState {
	query: string;
	ftsResults: SearchResult[];
	semanticResults: SemanticSearchResult[];
	loading: boolean;
	error: string | null;
	mode: 'fts' | 'semantic';
	queryTime: number;
}

function createSearchStore() {
	const initialState: SearchState = {
		query: '',
		ftsResults: [],
		semanticResults: [],
		loading: false,
		error: null,
		mode: 'fts',
		queryTime: 0
	};

	const { subscribe, set, update } = writable<SearchState>(initialState);

	return {
		subscribe,

		setMode(mode: 'fts' | 'semantic') {
			update((s) => ({ ...s, mode, ftsResults: [], semanticResults: [] }));
		},

		async search(query: string, entityTypes?: string[]) {
			if (!query.trim()) {
				update((s) => ({
					...s,
					query: '',
					ftsResults: [],
					semanticResults: [],
					error: null
				}));
				return;
			}

			update((s) => ({ ...s, query, loading: true, error: null }));

			try {
				let state: Partial<SearchState> = { loading: false };

				const currentState = await new Promise<SearchState>((resolve) => {
					const unsubscribe = subscribe((s) => {
						resolve(s);
						unsubscribe();
					});
				});

				if (currentState.mode === 'fts') {
					const response = await api.search.fts({
						query,
						entity_types: entityTypes,
						limit: 50
					});
					state = {
						...state,
						ftsResults: response.results,
						queryTime: response.query_time_ms
					};
				} else {
					const response = await api.search.semantic({
						query,
						entity_types: entityTypes,
						limit: 20,
						min_score: 0.5
					});
					state = {
						...state,
						semanticResults: response.results,
						queryTime: response.query_time_ms
					};
				}

				update((s) => ({ ...s, ...state }));
			} catch (e) {
				update((s) => ({
					...s,
					loading: false,
					error: e instanceof Error ? e.message : 'Search failed'
				}));
			}
		},

		clear() {
			set(initialState);
		}
	};
}

export const searchStore = createSearchStore();
