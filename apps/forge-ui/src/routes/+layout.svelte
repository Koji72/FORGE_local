<script lang="ts">
	import '../app.css';
	import Sidebar from '$components/Sidebar.svelte';
	import Titlebar from '$components/Titlebar.svelte';
	import Toast from '$components/Toast.svelte';
	import { toasts } from '$stores/toast';

	let { children } = $props();
</script>

<div class="flex h-full flex-col">
	<!-- Custom titlebar for Tauri -->
	<Titlebar />

	<div class="flex flex-1 overflow-hidden">
		<!-- Sidebar navigation -->
		<Sidebar />

		<!-- Main content area -->
		<main class="flex-1 overflow-auto bg-gray-50 dark:bg-gray-900">
			{@render children()}
		</main>
	</div>

	<!-- Toast notifications -->
	<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
		{#each $toasts as toast (toast.id)}
			<Toast {toast} />
		{/each}
	</div>
</div>
