<script lang="ts">
	import { X, CheckCircle, AlertCircle, AlertTriangle, Info } from 'lucide-svelte';
	import { removeToast, type Toast } from '$stores/toast';
	import clsx from 'clsx';

	let { toast }: { toast: Toast } = $props();

	const icons = {
		success: CheckCircle,
		error: AlertCircle,
		warning: AlertTriangle,
		info: Info
	};

	const colors = {
		success: 'bg-green-50 text-green-800 dark:bg-green-900/50 dark:text-green-300',
		error: 'bg-red-50 text-red-800 dark:bg-red-900/50 dark:text-red-300',
		warning: 'bg-yellow-50 text-yellow-800 dark:bg-yellow-900/50 dark:text-yellow-300',
		info: 'bg-blue-50 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300'
	};

	const iconColors = {
		success: 'text-green-500',
		error: 'text-red-500',
		warning: 'text-yellow-500',
		info: 'text-blue-500'
	};

	const Icon = icons[toast.type];
</script>

<div
	class={clsx(
		'flex w-80 items-start gap-3 rounded-lg border p-4 shadow-lg animate-slide-up',
		colors[toast.type],
		toast.type === 'success' && 'border-green-200 dark:border-green-800',
		toast.type === 'error' && 'border-red-200 dark:border-red-800',
		toast.type === 'warning' && 'border-yellow-200 dark:border-yellow-800',
		toast.type === 'info' && 'border-blue-200 dark:border-blue-800'
	)}
	role="alert"
>
	<Icon class={clsx('h-5 w-5 shrink-0', iconColors[toast.type])} />

	<div class="flex-1 min-w-0">
		{#if toast.title}
			<p class="font-medium">{toast.title}</p>
		{/if}
		{#if toast.message}
			<p class="text-sm opacity-90">{toast.message}</p>
		{/if}
	</div>

	<button
		onclick={() => removeToast(toast.id)}
		class="shrink-0 rounded p-1 opacity-70 hover:opacity-100"
		aria-label="Dismiss"
	>
		<X class="h-4 w-4" />
	</button>
</div>
