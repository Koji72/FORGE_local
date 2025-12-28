<script lang="ts">
	import { onMount } from 'svelte';
	import { tasks } from '$stores/tasks';
	import { CheckCircle, Circle, Clock, Inbox, Calendar } from 'lucide-svelte';

	let stats = $state({
		inbox: 0,
		today: 0,
		upcoming: 0,
		completed: 0
	});

	onMount(async () => {
		await tasks.load();
		// Calculate stats
		const now = Date.now();
		const endOfDay = new Date();
		endOfDay.setHours(23, 59, 59, 999);

		stats = {
			inbox: $tasks.filter((t) => t.status === 'inbox').length,
			today: $tasks.filter((t) => t.due_at && t.due_at <= endOfDay.getTime()).length,
			upcoming: $tasks.filter((t) => t.due_at && t.due_at > endOfDay.getTime()).length,
			completed: $tasks.filter((t) => t.status === 'done').length
		};
	});
</script>

<svelte:head>
	<title>Dashboard - Forge</title>
</svelte:head>

<div class="p-6">
	<div class="mb-8">
		<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Dashboard</h1>
		<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
			Welcome back. Here's an overview of your tasks.
		</p>
	</div>

	<!-- Stats Grid -->
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
		<a href="/tasks?status=inbox" class="card group hover:border-forge-500 transition-colors">
			<div class="flex items-center gap-4">
				<div class="rounded-lg bg-blue-100 p-3 dark:bg-blue-900">
					<Inbox class="h-6 w-6 text-blue-600 dark:text-blue-400" />
				</div>
				<div>
					<p class="text-sm font-medium text-gray-500 dark:text-gray-400">Inbox</p>
					<p class="text-2xl font-semibold text-gray-900 dark:text-white">{stats.inbox}</p>
				</div>
			</div>
		</a>

		<a href="/tasks?due=today" class="card group hover:border-forge-500 transition-colors">
			<div class="flex items-center gap-4">
				<div class="rounded-lg bg-orange-100 p-3 dark:bg-orange-900">
					<Clock class="h-6 w-6 text-orange-600 dark:text-orange-400" />
				</div>
				<div>
					<p class="text-sm font-medium text-gray-500 dark:text-gray-400">Due Today</p>
					<p class="text-2xl font-semibold text-gray-900 dark:text-white">{stats.today}</p>
				</div>
			</div>
		</a>

		<a href="/tasks?due=upcoming" class="card group hover:border-forge-500 transition-colors">
			<div class="flex items-center gap-4">
				<div class="rounded-lg bg-purple-100 p-3 dark:bg-purple-900">
					<Calendar class="h-6 w-6 text-purple-600 dark:text-purple-400" />
				</div>
				<div>
					<p class="text-sm font-medium text-gray-500 dark:text-gray-400">Upcoming</p>
					<p class="text-2xl font-semibold text-gray-900 dark:text-white">{stats.upcoming}</p>
				</div>
			</div>
		</a>

		<a href="/tasks?status=done" class="card group hover:border-forge-500 transition-colors">
			<div class="flex items-center gap-4">
				<div class="rounded-lg bg-green-100 p-3 dark:bg-green-900">
					<CheckCircle class="h-6 w-6 text-green-600 dark:text-green-400" />
				</div>
				<div>
					<p class="text-sm font-medium text-gray-500 dark:text-gray-400">Completed</p>
					<p class="text-2xl font-semibold text-gray-900 dark:text-white">{stats.completed}</p>
				</div>
			</div>
		</a>
	</div>

	<!-- Recent Tasks -->
	<div class="mt-8">
		<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Recent Tasks</h2>
		<div class="card">
			{#if $tasks.length === 0}
				<div class="text-center py-8 text-gray-500 dark:text-gray-400">
					<Circle class="h-12 w-12 mx-auto mb-4 opacity-50" />
					<p>No tasks yet. Create your first task to get started.</p>
					<a href="/tasks/new" class="btn-primary mt-4 inline-block">Create Task</a>
				</div>
			{:else}
				<ul class="divide-y divide-gray-200 dark:divide-gray-700">
					{#each $tasks.slice(0, 5) as task}
						<li class="py-3">
							<a href="/tasks/{task.id}" class="flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-700 -mx-4 px-4 py-2 rounded-lg transition-colors">
								{#if task.status === 'done'}
									<CheckCircle class="h-5 w-5 text-green-500" />
								{:else}
									<Circle class="h-5 w-5 text-gray-400" />
								{/if}
								<span class="flex-1 text-gray-900 dark:text-white" class:line-through={task.status === 'done'}>
									{task.title}
								</span>
								{#if task.due_at}
									<span class="text-sm text-gray-500">
										{new Date(task.due_at).toLocaleDateString()}
									</span>
								{/if}
							</a>
						</li>
					{/each}
				</ul>
				{#if $tasks.length > 5}
					<div class="mt-4 text-center">
						<a href="/tasks" class="text-sm text-forge-600 hover:text-forge-700">
							View all {$tasks.length} tasks →
						</a>
					</div>
				{/if}
			{/if}
		</div>
	</div>
</div>
