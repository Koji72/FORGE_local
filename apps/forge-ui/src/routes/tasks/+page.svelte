<script lang="ts">
	import { onMount } from 'svelte';
	import { Plus, Filter, Search, SortAsc } from 'lucide-svelte';
	import { taskStore, tasks } from '$stores/tasks';
	import type { TaskResponse } from '$api/types';
	import TaskCard from '$components/TaskCard.svelte';
	import TaskForm from '$components/TaskForm.svelte';

	let showNewForm = $state(false);
	let editingTask = $state<TaskResponse | undefined>(undefined);
	let searchQuery = $state('');
	let statusFilter = $state<string>('all');
	let sortBy = $state<'priority' | 'due_at' | 'created_at'>('priority');

	const statusOptions = [
		{ value: 'all', label: 'All' },
		{ value: 'inbox', label: 'Inbox' },
		{ value: 'next', label: 'Next' },
		{ value: 'waiting', label: 'Waiting' },
		{ value: 'someday', label: 'Someday' },
		{ value: 'done', label: 'Done' }
	];

	onMount(() => {
		taskStore.load();
	});

	let filteredTasks = $derived.by(() => {
		let result = [...$tasks];

		// Filter by status
		if (statusFilter !== 'all') {
			result = result.filter((t) => t.status === statusFilter);
		}

		// Filter by search
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(t) =>
					t.title.toLowerCase().includes(query) ||
					t.description_md?.toLowerCase().includes(query) ||
					t.context_tags?.some((tag) => tag.toLowerCase().includes(query))
			);
		}

		// Sort
		result.sort((a, b) => {
			if (sortBy === 'priority') {
				return (a.priority || 4) - (b.priority || 4);
			} else if (sortBy === 'due_at') {
				if (!a.due_at && !b.due_at) return 0;
				if (!a.due_at) return 1;
				if (!b.due_at) return -1;
				return a.due_at - b.due_at;
			} else {
				return b.created_at - a.created_at;
			}
		});

		return result;
	});

	function handleEdit(task: TaskResponse) {
		editingTask = task;
	}
</script>

<svelte:head>
	<title>Tasks | Forge</title>
</svelte:head>

<div class="flex h-full flex-col">
	<!-- Header -->
	<header class="flex items-center justify-between border-b bg-white px-6 py-4">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">Tasks</h1>
			<p class="mt-1 text-sm text-gray-500">
				{filteredTasks.length} task{filteredTasks.length === 1 ? '' : 's'}
			</p>
		</div>

		<button
			onclick={() => (showNewForm = true)}
			class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
		>
			<Plus class="h-4 w-4" />
			New Task
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
				placeholder="Search tasks..."
				class="w-full rounded-lg border border-gray-300 bg-white py-2 pl-10 pr-4 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
			/>
		</div>

		<!-- Status filter -->
		<div class="flex items-center gap-2">
			<Filter class="h-4 w-4 text-gray-400" />
			<select
				bind:value={statusFilter}
				class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
			>
				{#each statusOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>

		<!-- Sort -->
		<div class="flex items-center gap-2">
			<SortAsc class="h-4 w-4 text-gray-400" />
			<select
				bind:value={sortBy}
				class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
			>
				<option value="priority">Priority</option>
				<option value="due_at">Due Date</option>
				<option value="created_at">Created</option>
			</select>
		</div>
	</div>

	<!-- Task list -->
	<div class="flex-1 overflow-y-auto p-6">
		{#if $taskStore.loading}
			<div class="flex items-center justify-center py-12">
				<div class="h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent"></div>
			</div>
		{:else if $taskStore.error}
			<div class="rounded-lg bg-red-50 p-4 text-center text-red-600">
				{$taskStore.error}
			</div>
		{:else if filteredTasks.length === 0}
			<div class="py-12 text-center">
				<div class="mx-auto mb-4 h-16 w-16 rounded-full bg-gray-100 p-4">
					<svg class="h-8 w-8 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
						/>
					</svg>
				</div>
				<h3 class="text-lg font-medium text-gray-900">No tasks found</h3>
				<p class="mt-1 text-gray-500">
					{searchQuery || statusFilter !== 'all'
						? 'Try adjusting your filters'
						: 'Create your first task to get started'}
				</p>
				{#if !searchQuery && statusFilter === 'all'}
					<button
						onclick={() => (showNewForm = true)}
						class="mt-4 inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
					>
						<Plus class="h-4 w-4" />
						New Task
					</button>
				{/if}
			</div>
		{:else}
			<div class="space-y-3">
				{#each filteredTasks as task (task.id)}
					<TaskCard {task} onEdit={handleEdit} />
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- New/Edit Task Modal -->
{#if showNewForm}
	<TaskForm onClose={() => (showNewForm = false)} />
{/if}

{#if editingTask}
	<TaskForm task={editingTask} onClose={() => (editingTask = undefined)} />
{/if}
