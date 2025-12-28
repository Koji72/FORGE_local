import { writable, derived } from 'svelte/store';
import { api } from '$api/client';
import type { NoteResponse, ListNotesRequest } from '$api/types';

interface NoteState {
	items: NoteResponse[];
	loading: boolean;
	error: string | null;
	filter: ListNotesRequest;
}

function createNoteStore() {
	const initialState: NoteState = {
		items: [],
		loading: false,
		error: null,
		filter: {}
	};

	const { subscribe, set, update } = writable<NoteState>(initialState);

	return {
		subscribe,

		async load(filter?: ListNotesRequest) {
			update((s) => ({ ...s, loading: true, error: null }));

			try {
				const response = await api.notes.list(filter || {});
				update((s) => ({
					...s,
					items: response.notes,
					loading: false,
					filter: filter || {}
				}));
			} catch (e) {
				update((s) => ({
					...s,
					loading: false,
					error: e instanceof Error ? e.message : 'Failed to load notes'
				}));
			}
		},

		async create(data: { title: string; content_md: string; note_type?: string }) {
			try {
				const note = await api.notes.create(data);
				update((s) => ({ ...s, items: [note, ...s.items] }));
				return note;
			} catch (e) {
				throw e;
			}
		},

		async update(id: string, data: Partial<NoteResponse>) {
			try {
				await api.notes.update({ id, ...data });
				update((s) => ({
					...s,
					items: s.items.map((n) => (n.id === id ? { ...n, ...data } : n))
				}));
			} catch (e) {
				throw e;
			}
		},

		async delete(id: string) {
			try {
				await api.notes.delete({ id });
				update((s) => ({
					...s,
					items: s.items.filter((n) => n.id !== id)
				}));
			} catch (e) {
				throw e;
			}
		},

		async togglePin(id: string) {
			const currentNotes = await new Promise<NoteResponse[]>((resolve) => {
				const unsubscribe = subscribe((s) => {
					resolve(s.items);
					unsubscribe();
				});
			});

			const note = currentNotes.find((n) => n.id === id);
			if (!note) return;

			await this.update(id, { pinned: !note.pinned });
		}
	};
}

export const noteStore = createNoteStore();

// Derived store for just the notes array
export const notes = derived(noteStore, ($store) => $store.items);
