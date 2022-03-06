const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

// http://localhost:8080/webpack-dev-server  will show you what ws is loading. it is only index.js
// after using compy plugin, it is index.js and index.html
module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bootstrap.js",
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  plugins: [
    new CopyWebpackPlugin({
      // to is the output path
      patterns: [{ from: "./index.html", to: "./" }],
    }),
  ],
};
