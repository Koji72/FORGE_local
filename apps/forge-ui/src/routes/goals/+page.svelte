<script lang="ts">
	import { onMount } from 'svelte';
	import { Plus, Target, ChevronRight, MoreVertical, Edit, Trash2 } from 'lucide-svelte';
	import { goalStore, goals } from '$stores/goals';
	import type { GoalResponse } from '$api/types';
	import { toasts } from '$stores/toast';

	let showNewForm = $state(false);
	let editingGoal = $state<GoalResponse | undefined>(undefined);
	let horizonFilter = $state<string>('all');

	// Form state
	let formTitle = $state('');
	let formDescription = $state('');
	let formHorizon = $state('year');
	let formTargetDate = $state('');
	let saving = $state(false);

	const horizons = [
		{ value: 'all', label: 'All Horizons' },
		{ value: 'day', label: 'Daily' },
		{ value: 'week', label: 'Weekly' },
		{ value: 'month', label: 'Monthly' },
		{ value: 'quarter', label: 'Quarterly' },
		{ value: 'year', label: 'Yearly' },
		{ value: 'life', label: 'Life' }
	];

	const horizonColors: Record<string, string> = {
		day: 'bg-green-100 text-green-700',
		week: 'bg-blue-100 text-blue-700',
		month: 'bg-purple-100 text-purple-700',
		quarter: 'bg-orange-100 text-orange-700',
		year: 'bg-red-100 text-red-700',
		life: 'bg-gray-100 text-gray-700'
	};

	const statusColors: Record<string, string> = {
		active: 'bg-blue-100 text-blue-700',
		completed: 'bg-green-100 text-green-700',
		paused: 'bg-yellow-100 text-yellow-700',
		cancelled: 'bg-gray-100 text-gray-700'
	};

	onMount(() => {
		goalStore.load();
	});

	let filteredGoals = $derived.by(() => {
		let result = [...$goals];

		if (horizonFilter !== 'all') {
			result = result.filter((g) => g.horizon === horizonFilter);
		}

		// Sort by horizon (broader first) then by progress
		const horizonOrder = ['life', 'year', 'quarter', 'month', 'week', 'day'];
		result.sort((a, b) => {
			const aOrder = horizonOrder.indexOf(a.horizon);
			const bOrder = horizonOrder.indexOf(b.horizon);
			if (aOrder !== bOrder) return aOrder - bOrder;
			return b.progress_percent - a.progress_percent;
		});

		return result;
	});

	function openNewForm() {
		formTitle = '';
		formDescription = '';
		formHorizon = 'year';
		formTargetDate = '';
		editingGoal = undefined;
		showNewForm = true;
	}

	function openEditForm(goal: GoalResponse) {
		formTitle = goal.title;
		formDescription = goal.description_md || '';
		formHorizon = goal.horizon;
		formTargetDate = goal.target_date
			? new Date(goal.target_date).toISOString().split('T')[0]
			: '';
		editingGoal = goal;
		showNewForm = true;
	}

	function closeForm() {
		showNewForm = false;
		editingGoal = undefined;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!formTitle.trim()) return;

		saving = true;

		try {
			const data = {
				title: formTitle.trim(),
				description_md: formDescription.trim() || undefined,
				horizon: formHorizon,
				target_date: formTargetDate ? new Date(formTargetDate).getTime() : undefined
			};

			if (editingGoal) {
				await goalStore.update(editingGoal.id, data);
				toasts.success('Goal updated');
			} else {
				await goalStore.create(data);
				toasts.success('Goal created');
			}

			closeForm();
		} catch (e) {
			toasts.error(editingGoal ? 'Failed to update goal' : 'Failed to create goal');
		} finally {
			saving = false;
		}
	}

	async function deleteGoal(id: string) {
		try {
			await goalStore.delete(id);
			toasts.success('Goal deleted');
		} catch (e) {
			toasts.error('Failed to delete goal');
		}
	}

	async function updateProgress(goal: GoalResponse, delta: number) {
		const newProgress = Math.min(100, Math.max(0, goal.progress_percent + delta));
		try {
			await goalStore.updateProgress(goal.id, newProgress);
		} catch (e) {
			toasts.error('Failed to update progress');
		}
	}
</script>

<svelte:head>
	<title>Goals | Forge</title>
</svelte:head>

<div class="flex h-full flex-col">
	<!-- Header -->
	<header class="flex items-center justify-between border-b bg-white px-6 py-4">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">Goals</h1>
			<p class="mt-1 text-sm text-gray-500">
				{filteredGoals.length} goal{filteredGoals.length === 1 ? '' : 's'}
			</p>
		</div>

		<button
			onclick={openNewForm}
			class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
		>
			<Plus class="h-4 w-4" />
			New Goal
		</button>
	</header>

	<!-- Filters -->
	<div class="flex items-center gap-4 border-b bg-gray-50 px-6 py-3">
		<div class="flex items-center gap-2">
			<Target class="h-4 w-4 text-gray-400" />
			<select
				bind:value={horizonFilter}
				class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
			>
				{#each horizons as h}
					<option value={h.value}>{h.label}</option>
				{/each}
			</select>
		</div>
	</div>

	<!-- Goals list -->
	<div class="flex-1 overflow-y-auto p-6">
		{#if $goalStore.loading}
			<div class="flex items-center justify-center py-12">
				<div class="h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent"></div>
			</div>
		{:else if $goalStore.error}
			<div class="rounded-lg bg-red-50 p-4 text-center text-red-600">
				{$goalStore.error}
			</div>
		{:else if filteredGoals.length === 0}
			<div class="py-12 text-center">
				<div class="mx-auto mb-4 h-16 w-16 rounded-full bg-gray-100 p-4">
					<Target class="h-8 w-8 text-gray-400" />
				</div>
				<h3 class="text-lg font-medium text-gray-900">No goals found</h3>
				<p class="mt-1 text-gray-500">Create your first goal to get started</p>
				<button
					onclick={openNewForm}
					class="mt-4 inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
				>
					<Plus class="h-4 w-4" />
					New Goal
				</button>
			</div>
		{:else}
			<div class="space-y-4">
				{#each filteredGoals as goal (goal.id)}
					<div class="group rounded-lg border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 hover:shadow-sm">
						<div class="flex items-start justify-between">
							<div class="flex-1">
								<div class="flex items-center gap-2">
									<span class="rounded-full px-2 py-0.5 text-xs {horizonColors[goal.horizon]}">
										{goal.horizon}
									</span>
									<span class="rounded-full px-2 py-0.5 text-xs {statusColors[goal.status]}">
										{goal.status}
									</span>
								</div>

								<h3 class="mt-2 font-medium text-gray-900">{goal.title}</h3>

								{#if goal.description_md}
									<p class="mt-1 text-sm text-gray-600">{goal.description_md}</p>
								{/if}

								<!-- Progress bar -->
								<div class="mt-3 flex items-center gap-3">
									<div class="h-2 flex-1 overflow-hidden rounded-full bg-gray-200">
										<div
											class="h-full bg-blue-500 transition-all"
											style="width: {goal.progress_percent}%"
										></div>
									</div>
									<span class="text-sm font-medium text-gray-600">
										{goal.progress_percent}%
									</span>
								</div>

								<!-- Progress buttons -->
								<div class="mt-2 flex items-center gap-2">
									<button
										onclick={() => updateProgress(goal, -10)}
										class="rounded px-2 py-1 text-xs text-gray-600 hover:bg-gray-100"
									>
										-10%
									</button>
									<button
										onclick={() => updateProgress(goal, 10)}
										class="rounded px-2 py-1 text-xs text-gray-600 hover:bg-gray-100"
									>
										+10%
									</button>
									<button
										onclick={() => goalStore.updateProgress(goal.id, 100)}
										class="rounded px-2 py-1 text-xs text-green-600 hover:bg-green-50"
									>
										Complete
									</button>
								</div>
							</div>

							<!-- Actions -->
							<div class="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
								<button
									onclick={() => openEditForm(goal)}
									class="rounded p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
								>
									<Edit class="h-4 w-4" />
								</button>
								<button
									onclick={() => deleteGoal(goal.id)}
									class="rounded p-1.5 text-gray-400 hover:bg-red-50 hover:text-red-600"
								>
									<Trash2 class="h-4 w-4" />
								</button>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- New/Edit Goal Modal -->
{#if showNewForm}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={closeForm}>
		<div
			class="w-full max-w-lg rounded-xl bg-white shadow-xl"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="flex items-center justify-between border-b px-6 py-4">
				<h2 class="text-lg font-semibold text-gray-900">
					{editingGoal ? 'Edit Goal' : 'New Goal'}
				</h2>
				<button onclick={closeForm} class="rounded p-1 text-gray-400 hover:bg-gray-100">
					×
				</button>
			</div>

			<form onsubmit={handleSubmit} class="p-6">
				<div class="space-y-4">
					<div>
						<label for="title" class="block text-sm font-medium text-gray-700">Title</label>
						<input
							type="text"
							id="title"
							bind:value={formTitle}
							placeholder="What do you want to achieve?"
							class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							required
						/>
					</div>

					<div>
						<label for="description" class="block text-sm font-medium text-gray-700">Description</label>
						<textarea
							id="description"
							bind:value={formDescription}
							rows={3}
							placeholder="Add details about this goal"
							class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						></textarea>
					</div>

					<div class="grid grid-cols-2 gap-4">
						<div>
							<label for="horizon" class="block text-sm font-medium text-gray-700">Horizon</label>
							<select
								id="horizon"
								bind:value={formHorizon}
								class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							>
								{#each horizons.slice(1) as h}
									<option value={h.value}>{h.label}</option>
								{/each}
							</select>
						</div>

						<div>
							<label for="targetDate" class="block text-sm font-medium text-gray-700">Target Date</label>
							<input
								type="date"
								id="targetDate"
								bind:value={formTargetDate}
								class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
						</div>
					</div>
				</div>

				<div class="mt-6 flex justify-end gap-3">
					<button
						type="button"
						onclick={closeForm}
						class="rounded-lg px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={saving || !formTitle.trim()}
						class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50"
					>
						{saving ? 'Saving...' : editingGoal ? 'Update' : 'Create'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
