/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../**/*.{html,rs}"],
  theme: {
    extend: {
      fontFamily: {
        Ubuntu: ["Ubuntu", "sans-serif"],
      },
      colors: {
        grey: "#191B1C",
        blue: "#519AE5",
        "dark-blue": "#1B222A",
        white: "#E8E5E1",
        black: "#000000",
      },
    },
  },
  plugins: [],
};
