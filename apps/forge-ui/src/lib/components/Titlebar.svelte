<script lang="ts">
	import { Minus, Square, X } from 'lucide-svelte';

	// Check if we're in Tauri
	const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

	async function minimize() {
		if (isTauri) {
			const { getCurrentWindow } = await import('@tauri-apps/api/window');
			await getCurrentWindow().minimize();
		}
	}

	async function toggleMaximize() {
		if (isTauri) {
			const { getCurrentWindow } = await import('@tauri-apps/api/window');
			await getCurrentWindow().toggleMaximize();
		}
	}

	async function close() {
		if (isTauri) {
			const { getCurrentWindow } = await import('@tauri-apps/api/window');
			await getCurrentWindow().close();
		}
	}
</script>

{#if isTauri}
	<div
		data-tauri-drag-region
		class="flex h-8 items-center justify-end bg-white px-2 dark:bg-gray-800"
	>
		<div class="flex">
			<button
				onclick={minimize}
				class="inline-flex h-8 w-10 items-center justify-center text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-700"
				aria-label="Minimize"
			>
				<Minus class="h-4 w-4" />
			</button>
			<button
				onclick={toggleMaximize}
				class="inline-flex h-8 w-10 items-center justify-center text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-700"
				aria-label="Maximize"
			>
				<Square class="h-3.5 w-3.5" />
			</button>
			<button
				onclick={close}
				class="inline-flex h-8 w-10 items-center justify-center text-gray-500 hover:bg-red-500 hover:text-white"
				aria-label="Close"
			>
				<X class="h-4 w-4" />
			</button>
		</div>
	</div>
{/if}
