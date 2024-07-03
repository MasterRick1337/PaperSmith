module.exports = {
  content: [
          "./src/**/*.rs",
          "./index.html",
          "./src/**/*.html",
          "./tailwind.css",
      ],
  theme: {
    extend: {
      transitionProperty: {
        'max-height': 'max-height'
      }
    }
  },
  variants: {},
  plugins: [],
};