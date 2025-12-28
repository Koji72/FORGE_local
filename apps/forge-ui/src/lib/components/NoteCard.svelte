<script lang="ts">
	import { Pin, MoreVertical, Trash2, Edit, FileText } from 'lucide-svelte';
	import type { NoteResponse } from '$api/types';
	import { noteStore } from '$stores/notes';
	import { toasts } from '$stores/toast';

	let { note, onEdit }: { note: NoteResponse; onEdit?: (note: NoteResponse) => void } = $props();

	let showMenu = $state(false);

	const typeColors: Record<string, string> = {
		fleeting: 'bg-yellow-100 text-yellow-700',
		permanent: 'bg-green-100 text-green-700',
		literature: 'bg-blue-100 text-blue-700',
		reference: 'bg-purple-100 text-purple-700'
	};

	function formatDate(ts: number): string {
		return new Date(ts).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getPreview(content: string): string {
		// Strip markdown and get first 150 chars
		const plain = content
			.replace(/#{1,6}\s/g, '')
			.replace(/\*\*|__/g, '')
			.replace(/\*|_/g, '')
			.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
			.replace(/`{1,3}[^`]*`{1,3}/g, '')
			.trim();
		return plain.length > 150 ? plain.slice(0, 150) + '...' : plain;
	}

	async function togglePin() {
		try {
			await noteStore.togglePin(note.id);
		} catch (e) {
			toasts.error('Failed to update note');
		}
	}

	async function deleteNote() {
		try {
			await noteStore.delete(note.id);
			toasts.success('Note deleted');
		} catch (e) {
			toasts.error('Failed to delete note');
		}
		showMenu = false;
	}

	function handleEdit() {
		onEdit?.(note);
		showMenu = false;
	}
</script>

<div
	class="group relative flex flex-col rounded-lg border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 hover:shadow-sm"
>
	<!-- Header -->
	<div class="mb-2 flex items-start justify-between">
		<div class="flex items-center gap-2">
			<FileText class="h-4 w-4 text-gray-400" />
			{#if note.note_type}
				<span class="rounded-full px-2 py-0.5 text-xs {typeColors[note.note_type] || 'bg-gray-100 text-gray-700'}">
					{note.note_type}
				</span>
			{/if}
		</div>

		<div class="flex items-center gap-1">
			<!-- Pin button -->
			<button
				onclick={togglePin}
				class="rounded p-1 transition-opacity {note.pinned
					? 'text-amber-500'
					: 'text-gray-400 opacity-0 group-hover:opacity-100'} hover:bg-gray-100"
			>
				<Pin class="h-4 w-4" class:fill-current={note.pinned} />
			</button>

			<!-- Menu -->
			<div class="relative">
				<button
					onclick={() => (showMenu = !showMenu)}
					class="rounded p-1 text-gray-400 opacity-0 transition-opacity hover:bg-gray-100 hover:text-gray-600 group-hover:opacity-100"
				>
					<MoreVertical class="h-4 w-4" />
				</button>

				{#if showMenu}
					<div
						class="absolute right-0 top-full z-10 mt-1 w-36 rounded-lg border border-gray-200 bg-white py-1 shadow-lg"
					>
						<button
							onclick={handleEdit}
							class="flex w-full items-center gap-2 px-3 py-2 text-sm text-gray-700 hover:bg-gray-50"
						>
							<Edit class="h-4 w-4" />
							Edit
						</button>
						<button
							onclick={deleteNote}
							class="flex w-full items-center gap-2 px-3 py-2 text-sm text-red-600 hover:bg-red-50"
						>
							<Trash2 class="h-4 w-4" />
							Delete
						</button>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Title -->
	<h3 class="mb-1 font-medium text-gray-900">
		{note.title}
	</h3>

	<!-- Preview -->
	<p class="mb-3 line-clamp-3 flex-1 text-sm text-gray-600">
		{getPreview(note.content_md)}
	</p>

	<!-- Footer -->
	<div class="flex items-center justify-between text-xs text-gray-500">
		<span>{formatDate(note.updated_at)}</span>

		{#if note.tags?.length}
			<div class="flex flex-wrap gap-1">
				{#each note.tags.slice(0, 3) as tag}
					<span class="rounded-full bg-gray-100 px-2 py-0.5">#{tag}</span>
				{/each}
				{#if note.tags.length > 3}
					<span class="text-gray-400">+{note.tags.length - 3}</span>
				{/if}
			</div>
		{/if}
	</div>
</div>
