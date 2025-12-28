import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

	// Tauri expects a fixed port
	server: {
		port: 5173,
		strictPort: true,
		host: '127.0.0.1'
	},

	// Build configuration
	build: {
		target: 'esnext',
		minify: 'esbuild'
	},

	// Prevent Vite from obscuring Rust errors
	clearScreen: false,

	// Env prefix for Tauri
	envPrefix: ['VITE_', 'TAURI_']
});
