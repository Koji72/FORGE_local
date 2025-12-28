/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	darkMode: 'class',
	theme: {
		extend: {
			colors: {
				// Forge brand colors
				forge: {
					50: '#f5f7ff',
					100: '#ebf0fe',
					200: '#d6e0fd',
					300: '#b3c7fb',
					400: '#8aa6f7',
					500: '#6183f2',
					600: '#4361e8',
					700: '#354dd5',
					800: '#2e41ad',
					900: '#2a3b89',
					950: '#1c2553'
				}
			},
			fontFamily: {
				sans: ['Inter', 'system-ui', 'sans-serif'],
				mono: ['JetBrains Mono', 'Menlo', 'monospace']
			},
			animation: {
				'fade-in': 'fadeIn 0.2s ease-out',
				'slide-up': 'slideUp 0.2s ease-out',
				'slide-down': 'slideDown 0.2s ease-out'
			},
			keyframes: {
				fadeIn: {
					'0%': { opacity: '0' },
					'100%': { opacity: '1' }
				},
				slideUp: {
					'0%': { transform: 'translateY(10px)', opacity: '0' },
					'100%': { transform: 'translateY(0)', opacity: '1' }
				},
				slideDown: {
					'0%': { transform: 'translateY(-10px)', opacity: '0' },
					'100%': { transform: 'translateY(0)', opacity: '1' }
				}
			}
		}
	},
	plugins: []
};
