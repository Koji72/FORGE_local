<script lang="ts">
	import { Search, FileText, CheckSquare, Target, Zap, Clock } from 'lucide-svelte';
	import { searchStore } from '$stores/search';
	import { goto } from '$app/navigation';

	let searchInput = $state('');
	let entityFilter = $state<string[]>([]);
	let debounceTimer: ReturnType<typeof setTimeout>;

	const entityTypes = [
		{ value: 'task', label: 'Tasks', icon: CheckSquare },
		{ value: 'note', label: 'Notes', icon: FileText },
		{ value: 'goal', label: 'Goals', icon: Target }
	];

	function handleSearch() {
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			searchStore.search(searchInput, entityFilter.length > 0 ? entityFilter : undefined);
		}, 300);
	}

	function toggleEntityType(type: string) {
		if (entityFilter.includes(type)) {
			entityFilter = entityFilter.filter((t) => t !== type);
		} else {
			entityFilter = [...entityFilter, type];
		}
		handleSearch();
	}

	function navigateToResult(type: string, id: string) {
		switch (type) {
			case 'task':
				goto(`/tasks?id=${id}`);
				break;
			case 'note':
				goto(`/notes?id=${id}`);
				break;
			case 'goal':
				goto(`/goals?id=${id}`);
				break;
		}
	}

	function getEntityIcon(type: string) {
		switch (type) {
			case 'task':
				return CheckSquare;
			case 'note':
				return FileText;
			case 'goal':
				return Target;
			default:
				return FileText;
		}
	}

	function formatScore(score: number): string {
		return Math.round(score * 100) + '%';
	}
</script>

<svelte:head>
	<title>Search | Forge</title>
</svelte:head>

<div class="flex h-full flex-col">
	<!-- Header -->
	<header class="border-b bg-white px-6 py-4">
		<h1 class="text-2xl font-bold text-gray-900">Search</h1>
		<p class="mt-1 text-sm text-gray-500">Find tasks, notes, and goals</p>
	</header>

	<!-- Search input -->
	<div class="border-b bg-gray-50 px-6 py-4">
		<div class="relative">
			<Search class="absolute left-4 top-1/2 h-5 w-5 -translate-y-1/2 text-gray-400" />
			<input
				type="text"
				bind:value={searchInput}
				oninput={handleSearch}
				placeholder="Search everything..."
				class="w-full rounded-xl border border-gray-300 bg-white py-3 pl-12 pr-4 text-lg focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-200"
			/>
		</div>

		<!-- Filters -->
		<div class="mt-4 flex flex-wrap items-center gap-4">
			<!-- Entity type filters -->
			<div class="flex items-center gap-2">
				<span class="text-sm text-gray-500">Filter:</span>
				{#each entityTypes as type}
					<button
						onclick={() => toggleEntityType(type.value)}
						class="flex items-center gap-1.5 rounded-full px-3 py-1.5 text-sm transition-colors {entityFilter.includes(
							type.value
						)
							? 'bg-blue-100 text-blue-700'
							: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
					>
						<svelte:component this={type.icon} class="h-4 w-4" />
						{type.label}
					</button>
				{/each}
			</div>

			<!-- Search mode toggle -->
			<div class="flex items-center gap-2 rounded-lg border border-gray-300 bg-white p-1">
				<button
					onclick={() => {
						searchStore.setMode('fts');
						if (searchInput) handleSearch();
					}}
					class="flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm transition-colors {$searchStore.mode ===
					'fts'
						? 'bg-gray-100 text-gray-900'
						: 'text-gray-500 hover:text-gray-700'}"
				>
					<Search class="h-4 w-4" />
					Text
				</button>
				<button
					onclick={() => {
						searchStore.setMode('semantic');
						if (searchInput) handleSearch();
					}}
					class="flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm transition-colors {$searchStore.mode ===
					'semantic'
						? 'bg-gray-100 text-gray-900'
						: 'text-gray-500 hover:text-gray-700'}"
				>
					<Zap class="h-4 w-4" />
					Semantic
				</button>
			</div>
		</div>
	</div>

	<!-- Results -->
	<div class="flex-1 overflow-y-auto p-6">
		{#if $searchStore.loading}
			<div class="flex items-center justify-center py-12">
				<div class="h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent"></div>
			</div>
		{:else if $searchStore.error}
			<div class="rounded-lg bg-red-50 p-4 text-center text-red-600">
				{$searchStore.error}
			</div>
		{:else if !$searchStore.query}
			<div class="py-12 text-center">
				<div class="mx-auto mb-4 h-16 w-16 rounded-full bg-gray-100 p-4">
					<Search class="h-8 w-8 text-gray-400" />
				</div>
				<h3 class="text-lg font-medium text-gray-900">Start searching</h3>
				<p class="mt-1 text-gray-500">
					Type to search across all your tasks, notes, and goals
				</p>
			</div>
		{:else if $searchStore.mode === 'fts' && $searchStore.ftsResults.length === 0}
			<div class="py-12 text-center">
				<h3 class="text-lg font-medium text-gray-900">No results found</h3>
				<p class="mt-1 text-gray-500">Try different keywords or filters</p>
			</div>
		{:else if $searchStore.mode === 'semantic' && $searchStore.semanticResults.length === 0}
			<div class="py-12 text-center">
				<h3 class="text-lg font-medium text-gray-900">No results found</h3>
				<p class="mt-1 text-gray-500">Try a different query or lower the similarity threshold</p>
			</div>
		{:else}
			<!-- Query time -->
			<div class="mb-4 flex items-center gap-2 text-sm text-gray-500">
				<Clock class="h-4 w-4" />
				Found {$searchStore.mode === 'fts'
					? $searchStore.ftsResults.length
					: $searchStore.semanticResults.length} results in {$searchStore.queryTime}ms
			</div>

			<!-- FTS Results -->
			{#if $searchStore.mode === 'fts'}
				<div class="space-y-3">
					{#each $searchStore.ftsResults as result}
						<button
							onclick={() => navigateToResult(result.entity_type, result.entity_id)}
							class="block w-full rounded-lg border border-gray-200 bg-white p-4 text-left transition-all hover:border-gray-300 hover:shadow-sm"
						>
							<div class="flex items-start gap-3">
								<div class="rounded-lg bg-gray-100 p-2">
									<svelte:component this={getEntityIcon(result.entity_type)} class="h-5 w-5 text-gray-600" />
								</div>
								<div class="min-w-0 flex-1">
									<div class="flex items-center gap-2">
										<span class="rounded-full bg-gray-100 px-2 py-0.5 text-xs text-gray-600">
											{result.entity_type}
										</span>
										<span class="text-xs text-gray-400">
											Score: {formatScore(result.score)}
										</span>
									</div>
									<h3 class="mt-1 font-medium text-gray-900">{result.title}</h3>
									{#if result.snippet}
										<p class="mt-1 line-clamp-2 text-sm text-gray-600">
											{@html result.snippet}
										</p>
									{/if}
								</div>
							</div>
						</button>
					{/each}
				</div>
			{/if}

			<!-- Semantic Results -->
			{#if $searchStore.mode === 'semantic'}
				<div class="space-y-3">
					{#each $searchStore.semanticResults as result}
						<button
							onclick={() => navigateToResult(result.entity_type, result.entity_id)}
							class="block w-full rounded-lg border border-gray-200 bg-white p-4 text-left transition-all hover:border-gray-300 hover:shadow-sm"
						>
							<div class="flex items-start gap-3">
								<div class="rounded-lg bg-purple-100 p-2">
									<svelte:component this={getEntityIcon(result.entity_type)} class="h-5 w-5 text-purple-600" />
								</div>
								<div class="min-w-0 flex-1">
									<div class="flex items-center gap-2">
										<span class="rounded-full bg-purple-100 px-2 py-0.5 text-xs text-purple-600">
											{result.entity_type}
										</span>
										<span class="text-xs text-gray-400">
											Similarity: {formatScore(result.similarity)}
										</span>
									</div>
									<h3 class="mt-1 font-medium text-gray-900">{result.title}</h3>
								</div>
								<!-- Similarity bar -->
								<div class="flex items-center gap-2">
									<div class="h-2 w-20 overflow-hidden rounded-full bg-gray-200">
										<div
											class="h-full bg-purple-500"
											style="width: {result.similarity * 100}%"
										></div>
									</div>
								</div>
							</div>
						</button>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</div>
