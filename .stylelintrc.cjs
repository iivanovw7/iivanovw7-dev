module.exports = {
    plugins: ["stylelint-scss"],
    extends: ["stylelint-config-iivanovw7", "stylelint-config-recommended-scss"],
    ignoreFiles: ["**/*.js"],
    rules: {
        "import-notation": null,
        "alpha-value-notation": "percentage",
        "at-rule-no-unknown": null,
        "at-rule-empty-line-before": null,
        "scss/at-rule-no-unknown": true,
        "max-nesting-depth": [
            4,
            {
                ignore: ["pseudo-classes"],
            },
        ],
    },
};
