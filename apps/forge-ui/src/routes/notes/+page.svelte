<script lang="ts">
	import { onMount } from 'svelte';
	import { Plus, Filter, Search, Grid, List, Pin } from 'lucide-svelte';
	import { noteStore, notes } from '$stores/notes';
	import type { NoteResponse } from '$api/types';
	import NoteCard from '$components/NoteCard.svelte';
	import NoteForm from '$components/NoteForm.svelte';

	let showNewForm = $state(false);
	let editingNote = $state<NoteResponse | undefined>(undefined);
	let searchQuery = $state('');
	let typeFilter = $state<string>('all');
	let pinnedOnly = $state(false);
	let viewMode = $state<'grid' | 'list'>('grid');

	const typeOptions = [
		{ value: 'all', label: 'All Types' },
		{ value: 'fleeting', label: 'Fleeting' },
		{ value: 'permanent', label: 'Permanent' },
		{ value: 'literature', label: 'Literature' },
		{ value: 'reference', label: 'Reference' }
	];

	onMount(() => {
		noteStore.load();
	});

	let filteredNotes = $derived.by(() => {
		let result = [...$notes];

		// Filter by type
		if (typeFilter !== 'all') {
			result = result.filter((n) => n.note_type === typeFilter);
		}

		// Filter pinned only
		if (pinnedOnly) {
			result = result.filter((n) => n.pinned);
		}

		// Filter by search
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(n) =>
					n.title.toLowerCase().includes(query) ||
					n.content_md.toLowerCase().includes(query) ||
					n.tags?.some((tag) => tag.toLowerCase().includes(query))
			);
		}

		// Sort: pinned first, then by updated_at
		result.sort((a, b) => {
			if (a.pinned && !b.pinned) return -1;
			if (!a.pinned && b.pinned) return 1;
			return b.updated_at - a.updated_at;
		});

		return result;
	});

	function handleEdit(note: NoteResponse) {
		editingNote = note;
	}
</script>

<svelte:head>
	<title>Notes | Forge</title>
</svelte:head>

<div class="flex h-full flex-col">
	<!-- Header -->
	<header class="flex items-center justify-between border-b bg-white px-6 py-4">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">Notes</h1>
			<p class="mt-1 text-sm text-gray-500">
				{filteredNotes.length} note{filteredNotes.length === 1 ? '' : 's'}
			</p>
		</div>

		<button
			onclick={() => (showNewForm = true)}
			class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
		>
			<Plus class="h-4 w-4" />
			New Note
		</button>
	</header>

	<!-- Filters -->
	<div class="flex flex-wrap items-center gap-4 border-b bg-gray-50 px-6 py-3">
		<!-- Search -->
		<div class="relative flex-1">
			<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="Search notes..."
				class="w-full rounded-lg border border-gray-300 bg-white py-2 pl-10 pr-4 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
			/>
		</div>

		<!-- Type filter -->
		<div class="flex items-center gap-2">
			<Filter class="h-4 w-4 text-gray-400" />
			<select
				bind:value={typeFilter}
				class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
			>
				{#each typeOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>

		<!-- Pinned only toggle -->
		<button
			onclick={() => (pinnedOnly = !pinnedOnly)}
			class="flex items-center gap-2 rounded-lg border px-3 py-2 text-sm transition-colors {pinnedOnly
				? 'border-amber-300 bg-amber-50 text-amber-700'
				: 'border-gray-300 bg-white text-gray-700 hover:bg-gray-50'}"
		>
			<Pin class="h-4 w-4" class:fill-current={pinnedOnly} />
			Pinned
		</button>

		<!-- View mode toggle -->
		<div class="flex items-center rounded-lg border border-gray-300 bg-white">
			<button
				onclick={() => (viewMode = 'grid')}
				class="rounded-l-lg p-2 transition-colors {viewMode === 'grid'
					? 'bg-gray-100 text-gray-900'
					: 'text-gray-500 hover:text-gray-700'}"
			>
				<Grid class="h-4 w-4" />
			</button>
			<button
				onclick={() => (viewMode = 'list')}
				class="rounded-r-lg p-2 transition-colors {viewMode === 'list'
					? 'bg-gray-100 text-gray-900'
					: 'text-gray-500 hover:text-gray-700'}"
			>
				<List class="h-4 w-4" />
			</button>
		</div>
	</div>

	<!-- Note list -->
	<div class="flex-1 overflow-y-auto p-6">
		{#if $noteStore.loading}
			<div class="flex items-center justify-center py-12">
				<div class="h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent"></div>
			</div>
		{:else if $noteStore.error}
			<div class="rounded-lg bg-red-50 p-4 text-center text-red-600">
				{$noteStore.error}
			</div>
		{:else if filteredNotes.length === 0}
			<div class="py-12 text-center">
				<div class="mx-auto mb-4 h-16 w-16 rounded-full bg-gray-100 p-4">
					<svg class="h-8 w-8 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
						/>
					</svg>
				</div>
				<h3 class="text-lg font-medium text-gray-900">No notes found</h3>
				<p class="mt-1 text-gray-500">
					{searchQuery || typeFilter !== 'all' || pinnedOnly
						? 'Try adjusting your filters'
						: 'Create your first note to get started'}
				</p>
				{#if !searchQuery && typeFilter === 'all' && !pinnedOnly}
					<button
						onclick={() => (showNewForm = true)}
						class="mt-4 inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
					>
						<Plus class="h-4 w-4" />
						New Note
					</button>
				{/if}
			</div>
		{:else}
			<div
				class={viewMode === 'grid'
					? 'grid gap-4 sm:grid-cols-2 lg:grid-cols-3'
					: 'space-y-3'}
			>
				{#each filteredNotes as note (note.id)}
					<NoteCard {note} onEdit={handleEdit} />
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- New/Edit Note Modal -->
{#if showNewForm}
	<NoteForm onClose={() => (showNewForm = false)} />
{/if}

{#if editingNote}
	<NoteForm note={editingNote} onClose={() => (editingNote = undefined)} />
{/if}
