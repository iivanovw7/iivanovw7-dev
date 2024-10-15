import esbuild from "esbuild";
import fs from "node:fs";
import path from "node:path";
import url from "url";
import { sassPlugin } from "esbuild-sass-plugin";
import postcss from "postcss";
import postcssAutoprefixer from "autoprefixer";
import postcssDarkThemeClass from "postcss-dark-theme-class";
import postcssPresetEnv from "postcss-preset-env";
import postcss100vhFix from "postcss-100vh-fix";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const args = process.argv;
const isWatch = args[2]?.trim() == "--watchmode";
const mode = isWatch ? "context" : "build";

const postcssPlugins = [
    postcssDarkThemeClass({
        lightSelector: "[data-theme='light']",
        darkSelector: "[data-theme='dark']",
    }),
    postcss100vhFix,
    postcssAutoprefixer,
    postcssPresetEnv({
        stage: 0,
    }),
];

try {
    const ctx = await esbuild[mode]({
        bundle: true,
        minify: isWatch ? false : true,
        metafile: true,
        format: "esm",
        platform: "node",
        entryPoints: [path.resolve(__dirname, "./styles/main.scss")],
        outdir: path.resolve(__dirname, "./assets/css/"),
        entryNames: "[name]",
        plugins: [
            sassPlugin({
                async transform(source, _resolveDir) {
                    const { css } = await postcss(postcssPlugins).process(source, {
                        from: "*.scss",
                        to: "*.css",
                    });

                    return css;
                },
            }),
        ],
    });

    if (isWatch) {
        ctx.watch();
    } else {
        fs.writeFileSync(path.resolve(__dirname, "./assets/css/css.meta.json"), JSON.stringify(ctx.metafile));
    }
} catch (err) {
    console.error(err);
    process.exit(1);
}
