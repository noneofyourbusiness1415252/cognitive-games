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
    setupMiddlewares(middlewares, devServer) {
      if (!devServer) {
        throw new Error('webpack-dev-server is not defined');
      }
      
      // Add custom middleware to serve the favicon.ico file with SVG headers
      devServer.app.get('/favicon.ico', (req, res) => {
        res.setHeader('Content-Type', 'image/svg+xml');
        res.sendFile(resolve('static/favicon.svg'));
      });

      return middlewares;
    },
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
      extraArgs: "--weak-refs --reference-types",
    }),
  ]
}