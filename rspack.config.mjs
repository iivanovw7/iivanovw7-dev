import { defineConfig } from "@rspack/cli";
import { rspack } from "@rspack/core";
import autoprefixer from "autoprefixer";
import { CleanWebpackPlugin } from "clean-webpack-plugin";
import path from "node:path";
import postcss100vhFix from "postcss-100vh-fix";
import postcssDarkThemeClass from "postcss-dark-theme-class";
import postcssPresetEnv from "postcss-preset-env";
import { RspackManifestPlugin } from "rspack-manifest-plugin";
import * as sass from "sass-embedded";
import url from "url";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));

const postcssPlugins = [
	postcssDarkThemeClass({
		darkSelector: "[data-theme='dark']",
		lightSelector: "[data-theme='light']",
	}),
	postcss100vhFix,
	autoprefixer,
	postcssPresetEnv({
		stage: 0,
	}),
];

// eslint-disable-next-line import/no-default-export
export default defineConfig((env) => {
	const isWatch = env.RSPACK_WATCH;

	return {
		devtool: false,
		entry: {
			main: {
				import: path.resolve(__dirname, "./styles/main.scss"),
				layer: "styles",
			},
		},
		experiments: {
			css: true,
			layers: true,
		},
		mode: isWatch ? "development" : "production",
		module: {
			rules: [
				{
					generator: {
						filename: "[name].[contenthash].css",
					},
					test: /\.(sass|scss)$/,
					type: "asset/resource",
					use: [
						{
							loader: "postcss-loader",
							options: {
								postcssOptions: {
									plugins: postcssPlugins,
								},
							},
						},
						{
							loader: "sass-loader",
							options: {
								additionalData: `
                                    @use "@styles/abstracts" as *;
                                `,
								api: "modern-compiler",
								implementation: sass,
							},
						},
					],
				},
			],
		},
		optimization: {
			minimize: !isWatch,
		},
		output: {
			assetModuleFilename: "[name].[contenthash].[ext]",
			filename: () => "",
			path: path.resolve(__dirname, "./assets/css"),
		},
		plugins: [
			new CleanWebpackPlugin({
				cleanAfterEveryBuildPatterns: [],
				cleanOnceBeforeBuildPatterns: ["main.*.css", "main.*.js", "manifest.json"],
				verbose: true,
			}),
			new rspack.CssExtractRspackPlugin({
				assetModuleFilename: "[name][contenthash][ext]",
				filename: "[name].[contenthash].css",
				path: path.resolve(__dirname, "./assets/css"),
			}),
			new RspackManifestPlugin({
				fileName: "manifest.json",
				map: (file) => {
					file.path = file.path.replace("/assets/css/", "");

					return file;
				},
				publicPath: "/assets/css",
			}),
		],
		resolve: {
			alias: {
				"@styles": path.resolve(__dirname, "styles"),
				"@templates": path.resolve(__dirname, "templates"),
			},
		},
		watch: isWatch,
	};
});
