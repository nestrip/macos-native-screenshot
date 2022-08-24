/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.html"],
  theme: {
    extend: {
      colors: {
        primary: "#1A1B1E",
        secondary: "#25262b",
        border: "#5c5f66",
        text: "#C1C2C5",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
