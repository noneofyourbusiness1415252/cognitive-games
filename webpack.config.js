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
  },
  plugins: [
    new CopyPlugin(["static"]),
    new WasmPackPlugin({
      crateDirectory: ".",
    }),
  ]
}