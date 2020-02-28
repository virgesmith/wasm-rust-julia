const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: {
    julia: "./bootstrap_julia.js",
    mandel: "./bootstrap_mandel.js",
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap_[name].js",
  },
  mode: "production",
  plugins: [
    new CopyWebpackPlugin(['julia.html', 'mandel.html'])
  ],
};
