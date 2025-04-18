import { defineConfig } from "@rspack/cli";
import path from "node:path";
import url from "url";
import * as sass from "sass-embedded";
import { RspackManifestPlugin } from "rspack-manifest-plugin";

import autoprefixer from "autoprefixer";
import postcss100vhFix from "postcss-100vh-fix";
import postcssPresetEnv from "postcss-preset-env";
import postcssDarkThemeClass from "postcss-dark-theme-class";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));

const postcssPlugins = [
    postcssDarkThemeClass({
        lightSelector: "[data-theme='light']",
        darkSelector: "[data-theme='dark']",
    }),
    postcss100vhFix,
    autoprefixer,
    postcssPresetEnv({
        stage: 0,
    }),
];

export default defineConfig((env) => {
    const isWatch = env.RSPACK_WATCH;

    return {
        experiments: {
            css: true,
        },
        entry: {
            main: path.resolve(__dirname, "./styles/main.scss"),
        },
        optimization: {
            minimize: !isWatch,
        },
        output: {
            path: path.resolve(__dirname, "./assets/css/"),
            filename: "[name].[contenthash].js",
            assetModuleFilename: "[name][ext]",
        },
        resolve: {
            alias: {
                "@styles": path.resolve(__dirname, "styles"),
                "@templates": path.resolve(__dirname, "templates"),
            },
        },
        module: {
            rules: [
                {
                    test: /\.(sass|scss)$/,
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
                                api: "modern-compiler",
                                additionalData: `
                                    @use "@styles/abstracts" as *;
                                `,
                                implementation: sass,
                            },
                        },
                    ],
                    type: "css",
                },
            ],
        },
        plugins: [
            new RspackManifestPlugin({
                fileName: "manifest.json",
                publicPath: "/assets/css",
                map: (file) => {
                    file.path = file.path.replace("/assets/css/", "");

                    return file;
                },
            }),
        ],
        mode: isWatch ? "development" : "production",
        devtool: false,
        watch: isWatch,
    };
});
