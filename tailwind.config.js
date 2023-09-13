/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  prefix: "tw-",
  content: [
    `./src/**/*.rs`,
  ],
  plugins: [require("@mrvillage/tailwind"), require("@tailwindcss/typography")],
  corePlugins: {
    preflight: false,
  },
};
