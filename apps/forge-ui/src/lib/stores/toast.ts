import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
	id: string;
	type: ToastType;
	title?: string;
	message?: string;
	duration?: number;
}

function createToastStore() {
	const { subscribe, update } = writable<Toast[]>([]);

	function add(toast: Omit<Toast, 'id'>) {
		const id = crypto.randomUUID();
		const newToast: Toast = { id, ...toast };
		const duration = toast.duration ?? 5000;

		update((toasts) => [...toasts, newToast]);

		if (duration > 0) {
			setTimeout(() => {
				remove(id);
			}, duration);
		}

		return id;
	}

	function remove(id: string) {
		update((toasts) => toasts.filter((t) => t.id !== id));
	}

	function clear() {
		update(() => []);
	}

	return {
		subscribe,
		add,
		remove,
		clear,
		success: (title: string, message?: string) => add({ type: 'success', title, message }),
		error: (title: string, message?: string) => add({ type: 'error', title, message }),
		warning: (title: string, message?: string) => add({ type: 'warning', title, message }),
		info: (title: string, message?: string) => add({ type: 'info', title, message })
	};
}

export const toasts = createToastStore();

export function removeToast(id: string) {
	toasts.remove(id);
}
