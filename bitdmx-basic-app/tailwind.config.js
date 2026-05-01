/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './app/**/*.{js,ts,jsx,tsx}',
    './pages/**/*.{js,ts,jsx,tsx}',
    './components/**/*.{js,ts,jsx,tsx}',
    './lib/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Anonymous Pro"', 'ui-sans-serif', 'system-ui'],
        mono: ['"Anonymous Pro"', 'ui-monospace', 'SFMono-Regular'],
      },
    },
  },
  plugins: [],
}

