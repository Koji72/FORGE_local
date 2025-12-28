<script lang="ts">
	import { CheckCircle, Circle, Calendar, Clock, Flag, MoreVertical, Trash2, Edit } from 'lucide-svelte';
	import type { TaskResponse } from '$api/types';
	import { taskStore } from '$stores/tasks';
	import { toasts } from '$stores/toast';

	let { task, onEdit }: { task: TaskResponse; onEdit?: (task: TaskResponse) => void } = $props();

	let showMenu = $state(false);

	const priorityColors: Record<number, string> = {
		1: 'text-red-500',
		2: 'text-orange-500',
		3: 'text-yellow-500',
		4: 'text-blue-500',
		5: 'text-gray-400'
	};

	const statusColors: Record<string, string> = {
		inbox: 'bg-gray-100 text-gray-700',
		next: 'bg-blue-100 text-blue-700',
		waiting: 'bg-yellow-100 text-yellow-700',
		someday: 'bg-purple-100 text-purple-700',
		done: 'bg-green-100 text-green-700'
	};

	function formatDate(ts?: number): string {
		if (!ts) return '';
		return new Date(ts).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric'
		});
	}

	async function toggleComplete() {
		try {
			await taskStore.toggleComplete(task.id);
		} catch (e) {
			toasts.error('Failed to update task');
		}
	}

	async function deleteTask() {
		try {
			await taskStore.delete(task.id);
			toasts.success('Task deleted');
		} catch (e) {
			toasts.error('Failed to delete task');
		}
		showMenu = false;
	}

	function handleEdit() {
		onEdit?.(task);
		showMenu = false;
	}
</script>

<div
	class="group relative flex items-start gap-3 rounded-lg border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 hover:shadow-sm"
	class:opacity-60={task.status === 'done'}
>
	<!-- Checkbox -->
	<button
		onclick={toggleComplete}
		class="mt-0.5 flex-shrink-0 text-gray-400 transition-colors hover:text-green-500"
	>
		{#if task.status === 'done'}
			<CheckCircle class="h-5 w-5 text-green-500" />
		{:else}
			<Circle class="h-5 w-5" />
		{/if}
	</button>

	<!-- Content -->
	<div class="min-w-0 flex-1">
		<div class="flex items-start justify-between gap-2">
			<h3
				class="font-medium text-gray-900"
				class:line-through={task.status === 'done'}
				class:text-gray-500={task.status === 'done'}
			>
				{task.title}
			</h3>

			<!-- Priority flag -->
			{#if task.priority && task.priority <= 3}
				<Flag class="h-4 w-4 flex-shrink-0 {priorityColors[task.priority]}" />
			{/if}
		</div>

		{#if task.description_md}
			<p class="mt-1 line-clamp-2 text-sm text-gray-600">
				{task.description_md}
			</p>
		{/if}

		<!-- Meta info -->
		<div class="mt-2 flex flex-wrap items-center gap-3 text-xs text-gray-500">
			{#if task.due_at}
				<span class="flex items-center gap-1">
					<Calendar class="h-3 w-3" />
					{formatDate(task.due_at)}
				</span>
			{/if}

			{#if task.time_estimate_min}
				<span class="flex items-center gap-1">
					<Clock class="h-3 w-3" />
					{task.time_estimate_min}min
				</span>
			{/if}

			{#if task.status && task.status !== 'inbox'}
				<span class="rounded-full px-2 py-0.5 text-xs {statusColors[task.status] || statusColors.inbox}">
					{task.status}
				</span>
			{/if}

			{#if task.context_tags?.length}
				{#each task.context_tags.slice(0, 3) as tag}
					<span class="rounded-full bg-gray-100 px-2 py-0.5 text-gray-600">@{tag}</span>
				{/each}
			{/if}
		</div>
	</div>

	<!-- Actions menu -->
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
					onclick={deleteTask}
					class="flex w-full items-center gap-2 px-3 py-2 text-sm text-red-600 hover:bg-red-50"
				>
					<Trash2 class="h-4 w-4" />
					Delete
				</button>
			</div>
		{/if}
	</div>
</div>
