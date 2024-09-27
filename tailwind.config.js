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
      },
      colors: {
        rosewater: "rgba(245, 224, 220)",
        flamingo: "rgba(242, 205, 205)",
        pink: "rgba(245, 194, 231)",
        mauve: "rgba(203, 166, 247)",
        red: "rgba(243, 139, 168)",
        maroon: "rgba(235, 160, 172)",
        peach: "rgba(250, 179, 135)",
        yellow: "rgba(249, 226, 175)",
        green: "rgba(166, 227, 161)",
        teal: "rgba(148, 226, 213)",
        sky: "rgba(137, 220, 235)",
        sapphire: "rgba(116, 199, 236)",
        blue: "rgba(137, 180, 250)",
        lavender: "rgba(180, 190, 254)",
        text: "rgba(205, 214, 244)",
        subtext1: "rgba(186, 194, 222)",
        subtext0: "rgba(166, 173, 200)",
        overlay2: "rgba(147, 153, 178)",
        overlay1: "rgba(127, 132, 156)",
        overlay0: 'rgba(var(--overlay0))',
        surface2: 'rgba(88, 91, 112)',
        surface1: 'rgba(var(--surface1))',
        surface0: 'rgba(var(--surface0))',
        base: 'rgba(var(--base))',
        mantle: "rgba(24, 24, 37)",
        crust: "rgba(17, 17, 27)"
      },
    }
  },
  variants: {},
  plugins: [],
};