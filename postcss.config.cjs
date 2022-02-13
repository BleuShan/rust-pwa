const fontMagician = require('postcss-font-magician')
const preset = require('postcss-preset-env')

module.exports = {
  syntax: 'postcss-scss',
  plugins: [
    preset({
      stage: 0,
    }),
    fontMagician({
      display: 'swap',
      variants: {
        Roboto: {
          300: [],
          400: [],
          700: [],
        },
        'Material Icons': {
          400: [],
        },
      },
      foundries: 'google',
    }),
  ],
}
