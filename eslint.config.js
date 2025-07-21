import eslintImport from "eslint-plugin-import";
import jsdoc from "eslint-plugin-jsdoc";
import n from "eslint-plugin-n";
import nodeImport from "eslint-plugin-node-import";
import perfectionist from "eslint-plugin-perfectionist";
import preferArrow from "eslint-plugin-prefer-arrow";
import preferLet from "eslint-plugin-prefer-let";
import promise from "eslint-plugin-promise";
import sonarjs from "eslint-plugin-sonarjs";
import unicorn from "eslint-plugin-unicorn";
import globals from "globals";

// eslint-disable-next-line import/no-default-export
export default [
	perfectionist.configs["recommended-alphabetical"],
	jsdoc.configs["flat/recommended"],
	{
		ignores: [
			"**/node_modules/**",
			"**/dist/**",
			".git/**",
			"**/build/**",
			"**/dist/**",
			"**/target/**",
			"**/assets/**",
		],
	},
	{
		languageOptions: {
			ecmaVersion: "latest",
			globals: {
				...globals.browser,
				...globals.node,
			},
			sourceType: "module",
		},
		plugins: {
			import: eslintImport,
			jsdoc,
			n,
			"node-import": nodeImport,
			"prefer-arrow": preferArrow,
			"prefer-let": preferLet,
			promise,
			sonarjs,
			unicorn,
		},
		rules: {
			curly: ["error", "all"],
			"eol-last": "error",
			"import/consistent-type-specifier-style": ["error", "prefer-top-level"],
			"import/export": "error",
			"import/first": "error",
			"import/named": "error",
			"import/no-default-export": "warn",
			"import/no-duplicates": "off",
			"import/no-empty-named-blocks": "error",
			"import/no-extraneous-dependencies": [
				"error",
				{
					devDependencies: true,
					optionalDependencies: false,
					peerDependencies: false,
				},
			],
			"import/no-named-default": "error",
			"import/no-self-import": "error",
			"import/no-useless-path-segments": "error",
			"import/no-webpack-loader-syntax": "error",
			"import/prefer-default-export": "off",
			indent: ["error", "tab"],
			"linebreak-style": ["error", "unix"],
			// eslint-disable-next-line no-magic-numbers
			"max-len": ["error", 120],
			"n/handle-callback-err": ["error", "^(err|error)$"],
			"n/no-deprecated-api": "error",
			"n/no-exports-assign": "error",
			"n/no-path-concat": "error",
			"n/process-exit-as-throw": "error",
			"no-magic-numbers": [
				"error",
				{
					enforceConst: true,
					ignore: [-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 24, 60, 100, 1000],
					ignoreArrayIndexes: true,
					ignoreDefaultValues: true,
				},
			],
			"no-trailing-spaces": [
				"warn",
				{
					ignoreComments: true,
					skipBlankLines: true,
				},
			],
			"object-curly-newline": "off",
			"promise/no-multiple-resolved": "error",
			"promise/no-nesting": "error",
			"promise/no-promise-in-callback": "error",
			"promise/param-names": "error",
			"promise/valid-params": "error",
			quotes: ["error", "double"],
			semi: ["error", "always"],
			"semi-spacing": [
				"error",
				{
					after: true,
					before: false,
				},
			],
			"sonarjs/no-collapsible-if": "error",
			"sonarjs/no-duplicated-branches": "error",
			"sonarjs/no-identical-conditions": "error",
			"sonarjs/no-identical-functions": "error",
			"sonarjs/no-ignored-return": "error",
			"sonarjs/no-inverted-boolean-check": "error",
			"sonarjs/no-redundant-boolean": "error",
			"sonarjs/no-same-line-conditional": "error",
			"sonarjs/no-small-switch": "error",
			"sonarjs/prefer-immediate-return": "error",
			"sonarjs/prefer-single-boolean-return": "error",
			"unicorn/better-regex": "error",
			"unicorn/catch-error-name": "error",
			"unicorn/custom-error-definition": "error",
			"unicorn/no-for-loop": "error",
			"unicorn/no-instanceof-array": "error",
			"unicorn/no-invalid-remove-event-listener": "error",
			"unicorn/no-typeof-undefined": "error",
			"unicorn/no-unnecessary-await": "error",
			"unicorn/no-unused-properties": "error",
			"unicorn/no-useless-spread": "error",
			"unicorn/no-useless-undefined": [
				"error",
				{
					checkArguments: false,
				},
			],
			"unicorn/prefer-add-event-listener": "error",
			"unicorn/prefer-array-index-of": "error",
			"unicorn/prefer-array-some": "error",
			"unicorn/prefer-at": "error",
			"unicorn/prefer-date-now": "error",
			"unicorn/prefer-default-parameters": "error",
			"unicorn/prefer-includes": "error",
			"unicorn/prefer-keyboard-event-key": "error",
			"unicorn/prefer-logical-operator-over-ternary": "error",
			"unicorn/prefer-string-replace-all": "error",
			"unicorn/prefer-string-slice": "error",
			"unicorn/prefer-string-starts-ends-with": "error",
		},
	},
];
