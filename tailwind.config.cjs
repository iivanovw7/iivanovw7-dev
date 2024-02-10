/** @type { import("tailwindcss").Config } */
module.exports = {
    plugins: [
        require("@tailwindcss/aspect-ratio"),
        require("@tailwindcss/typography"),
        require("postcss-100vh-fix"),
        require("autoprefixer"),
    ],
    theme: {
        extend: {
            screens: {
                "hover-hover": { raw: "(hover: hover)" },
            },
        },
    },
    content: ["./templates/**/*.html"],
    darkMode: "class",
};
