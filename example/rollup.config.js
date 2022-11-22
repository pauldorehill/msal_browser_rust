import rust from "@wasm-tool/rollup-plugin-rust";
import serve from 'rollup-plugin-serve';
import liveReload from 'rollup-plugin-livereload';
import terser from "@rollup/plugin-terser";

const isdev = process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "js/",
            debug: isdev,
        }),
        isdev && serve({
            contentBase: "dist",
            verbose: true,
            open: true,
        }),
        isdev && liveReload({
            watch: "dist"
        }),
        !isdev && terser(),
    ],
};