import { resolve } from "path";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
export default {
  entry: "./js/index.js",
  output: {
    path: resolve("dist"),
    filename: "index.js"
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: "static" },
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "--weak-refs --reference-types",
      extraEnv: {
        RUSTUP_HOME: process.env.npm_config_rustup_home,
        CARGO_HOME: process.env.npm_config_cargo_home
      }
    }),
  ]
}