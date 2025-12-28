<script lang="ts">
	import { X } from 'lucide-svelte';
	import type { NoteResponse, CreateNoteRequest } from '$api/types';
	import { noteStore } from '$stores/notes';
	import { toasts } from '$stores/toast';

	let {
		note,
		onClose,
		onSave
	}: {
		note?: NoteResponse;
		onClose: () => void;
		onSave?: () => void;
	} = $props();

	let title = $state(note?.title || '');
	let content = $state(note?.content_md || '');
	let noteType = $state(note?.note_type || 'fleeting');
	let tags = $state(note?.tags?.join(', ') || '');
	let pinned = $state(note?.pinned || false);

	let saving = $state(false);

	const noteTypes = [
		{ value: 'fleeting', label: 'Fleeting', description: 'Quick thoughts and ideas' },
		{ value: 'permanent', label: 'Permanent', description: 'Refined, evergreen notes' },
		{ value: 'literature', label: 'Literature', description: 'Book/article notes' },
		{ value: 'reference', label: 'Reference', description: 'Reference material' }
	];

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!title.trim() || !content.trim()) return;

		saving = true;

		try {
			const data: CreateNoteRequest = {
				title: title.trim(),
				content_md: content.trim(),
				note_type: noteType,
				tags: tags
					? tags
							.split(',')
							.map((t) => t.trim())
							.filter(Boolean)
					: undefined,
				pinned
			};

			if (note) {
				await noteStore.update(note.id, data);
				toasts.success('Note updated');
			} else {
				await noteStore.create(data);
				toasts.success('Note created');
			}

			onSave?.();
			onClose();
		} catch (e) {
			toasts.error(note ? 'Failed to update note' : 'Failed to create note');
		} finally {
			saving = false;
		}
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={onClose}>
	<div
		class="flex h-[80vh] w-full max-w-3xl flex-col rounded-xl bg-white shadow-xl"
		onclick={(e) => e.stopPropagation()}
	>
		<!-- Header -->
		<div class="flex items-center justify-between border-b px-6 py-4">
			<h2 class="text-lg font-semibold text-gray-900">
				{note ? 'Edit Note' : 'New Note'}
			</h2>
			<button onclick={onClose} class="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600">
				<X class="h-5 w-5" />
			</button>
		</div>

		<!-- Form -->
		<form onsubmit={handleSubmit} class="flex flex-1 flex-col overflow-hidden p-6">
			<div class="flex-1 space-y-4 overflow-y-auto">
				<!-- Title -->
				<div>
					<label for="title" class="block text-sm font-medium text-gray-700">Title</label>
					<input
						type="text"
						id="title"
						bind:value={title}
						placeholder="Note title"
						class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						required
					/>
				</div>

				<!-- Content -->
				<div class="flex-1">
					<label for="content" class="block text-sm font-medium text-gray-700">Content</label>
					<textarea
						id="content"
						bind:value={content}
						placeholder="Write your note in Markdown..."
						class="mt-1 block h-64 w-full resize-none rounded-lg border border-gray-300 px-3 py-2 font-mono text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						required
					></textarea>
				</div>

				<!-- Type & Pinned row -->
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="noteType" class="block text-sm font-medium text-gray-700">Type</label>
						<select
							id="noteType"
							bind:value={noteType}
							class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						>
							{#each noteTypes as t}
								<option value={t.value}>{t.label}</option>
							{/each}
						</select>
					</div>

					<div class="flex items-end">
						<label class="flex items-center gap-2 text-sm font-medium text-gray-700">
							<input
								type="checkbox"
								bind:checked={pinned}
								class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
							/>
							Pin this note
						</label>
					</div>
				</div>

				<!-- Tags -->
				<div>
					<label for="tags" class="block text-sm font-medium text-gray-700">Tags</label>
					<input
						type="text"
						id="tags"
						bind:value={tags}
						placeholder="productivity, ideas, research (comma separated)"
						class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					/>
				</div>
			</div>

			<!-- Actions -->
			<div class="mt-6 flex justify-end gap-3 border-t pt-4">
				<button
					type="button"
					onclick={onClose}
					class="rounded-lg px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
				>
					Cancel
				</button>
				<button
					type="submit"
					disabled={saving || !title.trim() || !content.trim()}
					class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
				>
					{saving ? 'Saving...' : note ? 'Update' : 'Create'}
				</button>
			</div>
		</form>
	</div>
</div>
