<script lang="ts">
	import { X } from 'lucide-svelte';
	import type { TaskResponse, CreateTaskRequest } from '$api/types';
	import { taskStore } from '$stores/tasks';
	import { toasts } from '$stores/toast';

	let {
		task,
		onClose,
		onSave
	}: {
		task?: TaskResponse;
		onClose: () => void;
		onSave?: () => void;
	} = $props();

	let title = $state(task?.title || '');
	let description = $state(task?.description_md || '');
	let status = $state(task?.status || 'inbox');
	let priority = $state(task?.priority || 4);
	let dueDate = $state(task?.due_at ? new Date(task.due_at).toISOString().split('T')[0] : '');
	let timeEstimate = $state(task?.time_estimate_min?.toString() || '');
	let contextTags = $state(task?.context_tags?.join(', ') || '');
	let energyLevel = $state(task?.energy_level || '');

	let saving = $state(false);

	const statuses = [
		{ value: 'inbox', label: 'Inbox' },
		{ value: 'next', label: 'Next' },
		{ value: 'waiting', label: 'Waiting' },
		{ value: 'someday', label: 'Someday' },
		{ value: 'done', label: 'Done' }
	];

	const priorities = [
		{ value: 1, label: 'Urgent' },
		{ value: 2, label: 'High' },
		{ value: 3, label: 'Medium' },
		{ value: 4, label: 'Normal' },
		{ value: 5, label: 'Low' }
	];

	const energyLevels = [
		{ value: '', label: 'Any' },
		{ value: 'high', label: 'High' },
		{ value: 'medium', label: 'Medium' },
		{ value: 'low', label: 'Low' }
	];

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!title.trim()) return;

		saving = true;

		try {
			const data: CreateTaskRequest = {
				title: title.trim(),
				description_md: description.trim() || undefined,
				status,
				priority,
				due_at: dueDate ? new Date(dueDate).getTime() : undefined,
				time_estimate_min: timeEstimate ? parseInt(timeEstimate) : undefined,
				context_tags: contextTags
					? contextTags
							.split(',')
							.map((t) => t.trim())
							.filter(Boolean)
					: undefined,
				energy_level: energyLevel || undefined
			};

			if (task) {
				await taskStore.update(task.id, data);
				toasts.success('Task updated');
			} else {
				await taskStore.create(data);
				toasts.success('Task created');
			}

			onSave?.();
			onClose();
		} catch (e) {
			toasts.error(task ? 'Failed to update task' : 'Failed to create task');
		} finally {
			saving = false;
		}
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={onClose}>
	<div
		class="w-full max-w-lg rounded-xl bg-white shadow-xl"
		onclick={(e) => e.stopPropagation()}
	>
		<!-- Header -->
		<div class="flex items-center justify-between border-b px-6 py-4">
			<h2 class="text-lg font-semibold text-gray-900">
				{task ? 'Edit Task' : 'New Task'}
			</h2>
			<button onclick={onClose} class="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600">
				<X class="h-5 w-5" />
			</button>
		</div>

		<!-- Form -->
		<form onsubmit={handleSubmit} class="p-6">
			<div class="space-y-4">
				<!-- Title -->
				<div>
					<label for="title" class="block text-sm font-medium text-gray-700">Title</label>
					<input
						type="text"
						id="title"
						bind:value={title}
						placeholder="What needs to be done?"
						class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						required
					/>
				</div>

				<!-- Description -->
				<div>
					<label for="description" class="block text-sm font-medium text-gray-700">Description</label>
					<textarea
						id="description"
						bind:value={description}
						rows={3}
						placeholder="Add details (supports Markdown)"
						class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					></textarea>
				</div>

				<!-- Status & Priority row -->
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="status" class="block text-sm font-medium text-gray-700">Status</label>
						<select
							id="status"
							bind:value={status}
							class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						>
							{#each statuses as s}
								<option value={s.value}>{s.label}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="priority" class="block text-sm font-medium text-gray-700">Priority</label>
						<select
							id="priority"
							bind:value={priority}
							class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						>
							{#each priorities as p}
								<option value={p.value}>{p.label}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Due date & Time estimate row -->
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="dueDate" class="block text-sm font-medium text-gray-700">Due Date</label>
						<input
							type="date"
							id="dueDate"
							bind:value={dueDate}
							class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						/>
					</div>

					<div>
						<label for="timeEstimate" class="block text-sm font-medium text-gray-700">
							Time Estimate (min)
						</label>
						<input
							type="number"
							id="timeEstimate"
							bind:value={timeEstimate}
							min="1"
							placeholder="30"
							class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						/>
					</div>
				</div>

				<!-- Context tags -->
				<div>
					<label for="contextTags" class="block text-sm font-medium text-gray-700">
						Context Tags
					</label>
					<input
						type="text"
						id="contextTags"
						bind:value={contextTags}
						placeholder="home, work, phone (comma separated)"
						class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					/>
				</div>

				<!-- Energy level -->
				<div>
					<label for="energyLevel" class="block text-sm font-medium text-gray-700">
						Energy Level Required
					</label>
					<select
						id="energyLevel"
						bind:value={energyLevel}
						class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					>
						{#each energyLevels as e}
							<option value={e.value}>{e.label}</option>
						{/each}
					</select>
				</div>
			</div>

			<!-- Actions -->
			<div class="mt-6 flex justify-end gap-3">
				<button
					type="button"
					onclick={onClose}
					class="rounded-lg px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
				>
					Cancel
				</button>
				<button
					type="submit"
					disabled={saving || !title.trim()}
					class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
				>
					{saving ? 'Saving...' : task ? 'Update' : 'Create'}
				</button>
			</div>
		</form>
	</div>
</div>
