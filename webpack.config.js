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
  devServer: {
    static: "dist",
    open: true,
    port: 80,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: "static" },
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: ".",
    }),
  ]
}