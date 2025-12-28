<script lang="ts">
	import { onMount } from 'svelte';
	import { Settings, Database, Shield, Bell, Palette, Save, RefreshCw } from 'lucide-svelte';
	import { api } from '$api/client';
	import { toasts } from '$stores/toast';

	let loading = $state(true);
	let saving = $state(false);
	let settings = $state<Record<string, unknown>>({});

	// Local state for form
	let theme = $state<'light' | 'dark' | 'system'>('system');
	let autoBackup = $state(true);
	let backupInterval = $state(24);
	let enableSync = $state(false);
	let syncServer = $state('');
	let enableNotifications = $state(true);
	let defaultTaskStatus = $state('inbox');
	let defaultNoteType = $state('fleeting');

	onMount(async () => {
		try {
			const response = await api.settings.getAll();
			settings = response.settings;

			// Apply loaded settings
			theme = (settings['ui.theme'] as typeof theme) || 'system';
			autoBackup = (settings['backup.auto'] as boolean) ?? true;
			backupInterval = (settings['backup.interval_hours'] as number) || 24;
			enableSync = (settings['sync.enabled'] as boolean) ?? false;
			syncServer = (settings['sync.server_url'] as string) || '';
			enableNotifications = (settings['notifications.enabled'] as boolean) ?? true;
			defaultTaskStatus = (settings['defaults.task_status'] as string) || 'inbox';
			defaultNoteType = (settings['defaults.note_type'] as string) || 'fleeting';
		} catch (e) {
			toasts.error('Failed to load settings');
		} finally {
			loading = false;
		}
	});

	async function saveSetting(key: string, value: unknown) {
		try {
			await api.settings.set(key, value);
			settings[key] = value;
		} catch (e) {
			throw e;
		}
	}

	async function saveAllSettings() {
		saving = true;

		try {
			await Promise.all([
				saveSetting('ui.theme', theme),
				saveSetting('backup.auto', autoBackup),
				saveSetting('backup.interval_hours', backupInterval),
				saveSetting('sync.enabled', enableSync),
				saveSetting('sync.server_url', syncServer),
				saveSetting('notifications.enabled', enableNotifications),
				saveSetting('defaults.task_status', defaultTaskStatus),
				saveSetting('defaults.note_type', defaultNoteType)
			]);

			toasts.success('Settings saved');
		} catch (e) {
			toasts.error('Failed to save settings');
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>Settings | Forge</title>
</svelte:head>

<div class="flex h-full flex-col">
	<!-- Header -->
	<header class="flex items-center justify-between border-b bg-white px-6 py-4">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">Settings</h1>
			<p class="mt-1 text-sm text-gray-500">Configure your Forge experience</p>
		</div>

		<button
			onclick={saveAllSettings}
			disabled={saving}
			class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700 disabled:opacity-50"
		>
			{#if saving}
				<RefreshCw class="h-4 w-4 animate-spin" />
				Saving...
			{:else}
				<Save class="h-4 w-4" />
				Save Changes
			{/if}
		</button>
	</header>

	<!-- Settings content -->
	<div class="flex-1 overflow-y-auto p-6">
		{#if loading}
			<div class="flex items-center justify-center py-12">
				<div class="h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent"></div>
			</div>
		{:else}
			<div class="mx-auto max-w-2xl space-y-8">
				<!-- Appearance -->
				<section class="rounded-lg border border-gray-200 bg-white">
					<div class="flex items-center gap-3 border-b px-6 py-4">
						<Palette class="h-5 w-5 text-gray-500" />
						<h2 class="font-semibold text-gray-900">Appearance</h2>
					</div>
					<div class="p-6">
						<div>
							<label for="theme" class="block text-sm font-medium text-gray-700">Theme</label>
							<select
								id="theme"
								bind:value={theme}
								class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							>
								<option value="light">Light</option>
								<option value="dark">Dark</option>
								<option value="system">System</option>
							</select>
							<p class="mt-1 text-sm text-gray-500">Choose your preferred color scheme</p>
						</div>
					</div>
				</section>

				<!-- Data & Backup -->
				<section class="rounded-lg border border-gray-200 bg-white">
					<div class="flex items-center gap-3 border-b px-6 py-4">
						<Database class="h-5 w-5 text-gray-500" />
						<h2 class="font-semibold text-gray-900">Data & Backup</h2>
					</div>
					<div class="space-y-4 p-6">
						<div class="flex items-center justify-between">
							<div>
								<label for="autoBackup" class="font-medium text-gray-900">Automatic Backup</label>
								<p class="text-sm text-gray-500">Automatically backup your data</p>
							</div>
							<input
								type="checkbox"
								id="autoBackup"
								bind:checked={autoBackup}
								class="h-5 w-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
							/>
						</div>

						{#if autoBackup}
							<div>
								<label for="backupInterval" class="block text-sm font-medium text-gray-700">
									Backup Interval (hours)
								</label>
								<input
									type="number"
									id="backupInterval"
									bind:value={backupInterval}
									min="1"
									max="168"
									class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								/>
							</div>
						{/if}
					</div>
				</section>

				<!-- Sync -->
				<section class="rounded-lg border border-gray-200 bg-white">
					<div class="flex items-center gap-3 border-b px-6 py-4">
						<Shield class="h-5 w-5 text-gray-500" />
						<h2 class="font-semibold text-gray-900">Sync</h2>
					</div>
					<div class="space-y-4 p-6">
						<div class="flex items-center justify-between">
							<div>
								<label for="enableSync" class="font-medium text-gray-900">Enable Sync</label>
								<p class="text-sm text-gray-500">Sync data across devices with E2E encryption</p>
							</div>
							<input
								type="checkbox"
								id="enableSync"
								bind:checked={enableSync}
								class="h-5 w-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
							/>
						</div>

						{#if enableSync}
							<div>
								<label for="syncServer" class="block text-sm font-medium text-gray-700">
									Sync Server URL
								</label>
								<input
									type="url"
									id="syncServer"
									bind:value={syncServer}
									placeholder="wss://sync.example.com"
									class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								/>
							</div>
						{/if}
					</div>
				</section>

				<!-- Notifications -->
				<section class="rounded-lg border border-gray-200 bg-white">
					<div class="flex items-center gap-3 border-b px-6 py-4">
						<Bell class="h-5 w-5 text-gray-500" />
						<h2 class="font-semibold text-gray-900">Notifications</h2>
					</div>
					<div class="p-6">
						<div class="flex items-center justify-between">
							<div>
								<label for="enableNotifications" class="font-medium text-gray-900">
									Enable Notifications
								</label>
								<p class="text-sm text-gray-500">Get notified about due tasks and reminders</p>
							</div>
							<input
								type="checkbox"
								id="enableNotifications"
								bind:checked={enableNotifications}
								class="h-5 w-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
							/>
						</div>
					</div>
				</section>

				<!-- Defaults -->
				<section class="rounded-lg border border-gray-200 bg-white">
					<div class="flex items-center gap-3 border-b px-6 py-4">
						<Settings class="h-5 w-5 text-gray-500" />
						<h2 class="font-semibold text-gray-900">Defaults</h2>
					</div>
					<div class="space-y-4 p-6">
						<div>
							<label for="defaultTaskStatus" class="block text-sm font-medium text-gray-700">
								Default Task Status
							</label>
							<select
								id="defaultTaskStatus"
								bind:value={defaultTaskStatus}
								class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							>
								<option value="inbox">Inbox</option>
								<option value="next">Next</option>
								<option value="someday">Someday</option>
							</select>
						</div>

						<div>
							<label for="defaultNoteType" class="block text-sm font-medium text-gray-700">
								Default Note Type
							</label>
							<select
								id="defaultNoteType"
								bind:value={defaultNoteType}
								class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							>
								<option value="fleeting">Fleeting</option>
								<option value="permanent">Permanent</option>
								<option value="literature">Literature</option>
								<option value="reference">Reference</option>
							</select>
						</div>
					</div>
				</section>

				<!-- Version info -->
				<div class="text-center text-sm text-gray-500">
					<p>Forge v1.0.0</p>
					<p class="mt-1">Made with ❤️ for productivity</p>
				</div>
			</div>
		{/if}
	</div>
</div>
