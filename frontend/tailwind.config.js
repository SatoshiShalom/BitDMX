/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        bitcoin: {
          orange: '#F7931A',
          dark: '#1A1A1A',
          light: '#F8F9FA',
        },
      },
    },
  },
  plugins: [],
}
