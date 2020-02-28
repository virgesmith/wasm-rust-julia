const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

// change below:
// bootstrap.js -> bootstrap_mandel.js
// index.html -> mandel.html
// stop, rebuild, start
// navigate to /mandel.html

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
