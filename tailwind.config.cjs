const defaultTheme = require("tailwindcss/defaultTheme");

/** @type { import("tailwindcss").Config } */
module.exports = {
    plugins: [
        require("@tailwindcss/aspect-ratio"),
        require("@tailwindcss/typography"),
        require("postcss-100vh-fix"),
        require("autoprefixer"),
    ],
    content: ["./templates/**/*.html"],
    darkMode: "class",
    theme: {
        extend: {
            fontFamily: {
                sans: ['"IBM Plex Mono"', ...defaultTheme.fontFamily.sans],
            },
        },
    },
};
