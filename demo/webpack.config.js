/**
 * Webpack configuration for the jarsdraw demo.
 *
 * Bundles bootstrap.js as the entry point, copies index.html to dist/,
 * enables async WebAssembly, and watches the compiled pkg/ files so the
 * dev server reloads when wasm-pack rebuilds the Rust library.
 */
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    mode: "development",
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    watchOptions: {
        aggregateTimeout: 200,
        poll: 1000
    },
    devServer: {
        hot: false,
        liveReload: true,
        watchFiles: [
            path.resolve(__dirname, "*.js"),
            path.resolve(__dirname, "*.html"),
            path.resolve(__dirname, "pkg/*.js"),
            path.resolve(__dirname, "pkg/*.wasm")
        ]
    },
    plugins: [
        new CopyWebpackPlugin({ patterns: ['index.html'] })
    ],
    experiments: {
        asyncWebAssembly: true
    },
};
