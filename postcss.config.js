module.exports = {
  plugins: {
    "postcss-import": {},
    "tailwindcss/nesting": {},
    "tailwindcss": {},
    "autoprefixer": {},
    'postcss-replace': {
      pattern: /(--tw|\*, ::before, ::after)/g,
      data: {
        '--tw': '--mu-tw', // Prefixing
        '*, ::before, ::after': ':root', // So variables does not pollute every element
      }
    }
  }
}
