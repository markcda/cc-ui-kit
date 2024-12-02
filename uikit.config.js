module.exports = {
  mode: "all",
  content: [
    "./ui-kit/src/**/*.{rs,html,css}",
  ],
  theme: {
    extend: {
      keyframes: {
        "slide-in": {
          "0%": { opacity: 0, transform: "translateX(100%)" },
          "100%": { opacity: 1, transform: "translateX(0)" },
        },
        "accordion-down": {
          from: { "max-height": "0px" },
          to: { "max-height": "100px" },
        },
        "accordion-up": {
          from: { "max-height": "100px" },
          to: { "max-height": "0px" },
        },
      },
      animation: {
        "slide-in": "slide-in .25s ease-in-out forwards 1",
        "accordion-down": "accordion-down 1s ease-out",
        "accordion-up": "accordion-up 1s ease-out",
      },
    },
  },
  plugins: [],
};
