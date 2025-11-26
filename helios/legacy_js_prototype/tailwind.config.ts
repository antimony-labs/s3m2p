/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './app/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        'cosmic-indigo': '#0B0F1A',
        'cosmic-cyan': '#99E6FF',
      },
    },
  },
  plugins: [],
}




