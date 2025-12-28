<script lang="ts">
	import { page } from '$app/stores';
	import {
		Home,
		CheckSquare,
		FileText,
		Target,
		Search,
		Settings,
		Sparkles,
		ChevronLeft,
		ChevronRight
	} from 'lucide-svelte';
	import clsx from 'clsx';

	let collapsed = $state(false);

	const navItems = [
		{ href: '/', icon: Home, label: 'Dashboard' },
		{ href: '/tasks', icon: CheckSquare, label: 'Tasks' },
		{ href: '/notes', icon: FileText, label: 'Notes' },
		{ href: '/goals', icon: Target, label: 'Goals' },
		{ href: '/search', icon: Search, label: 'Search' },
		{ href: '/agents', icon: Sparkles, label: 'AI Agents' }
	];

	function isActive(href: string, pathname: string): boolean {
		if (href === '/') return pathname === '/';
		return pathname.startsWith(href);
	}
</script>

<aside
	class={clsx(
		'flex flex-col border-r border-gray-200 bg-white transition-all duration-200 dark:border-gray-700 dark:bg-gray-800',
		collapsed ? 'w-16' : 'w-64'
	)}
>
	<!-- Logo -->
	<div class="flex h-14 items-center justify-between px-4">
		{#if !collapsed}
			<span class="text-xl font-bold text-forge-600">Forge</span>
		{/if}
		<button
			onclick={() => (collapsed = !collapsed)}
			class="rounded-lg p-1.5 text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700"
			aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
		>
			{#if collapsed}
				<ChevronRight class="h-5 w-5" />
			{:else}
				<ChevronLeft class="h-5 w-5" />
			{/if}
		</button>
	</div>

	<!-- Navigation -->
	<nav class="flex-1 space-y-1 px-2 py-4">
		{#each navItems as item}
			{@const active = isActive(item.href, $page.url.pathname)}
			<a
				href={item.href}
				class={clsx(
					'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
					active
						? 'bg-forge-50 text-forge-700 dark:bg-forge-900/50 dark:text-forge-400'
						: 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
				)}
				title={collapsed ? item.label : undefined}
			>
				<item.icon class="h-5 w-5 shrink-0" />
				{#if !collapsed}
					<span>{item.label}</span>
				{/if}
			</a>
		{/each}
	</nav>

	<!-- Settings link at bottom -->
	<div class="border-t border-gray-200 px-2 py-4 dark:border-gray-700">
		{@const active = isActive('/settings', $page.url.pathname)}
		<a
			href="/settings"
			class={clsx(
				'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
				active
					? 'bg-forge-50 text-forge-700 dark:bg-forge-900/50 dark:text-forge-400'
					: 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
			)}
			title={collapsed ? 'Settings' : undefined}
		>
			<Settings class="h-5 w-5 shrink-0" />
			{#if !collapsed}
				<span>Settings</span>
			{/if}
		</a>
	</div>
</aside>
