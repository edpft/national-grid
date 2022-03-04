const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const docs = path.resolve(__dirname, "docs");

module.exports = {
  mode: "production",
  entry: ["./js/main.js", "./js/tabs.js", "./pkg/index.js"],
  output: {
    publicPath: "",
    path: docs,
    filename: "bundle.js"
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: path.resolve(__dirname, "static"), to: docs },
        { from: path.resolve(__dirname, "style"), to: docs },
      ],
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'src'),
      extraArgs: '--no-typescript',
    }),
  ]
};