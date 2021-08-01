const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const fs = require('fs');

module.exports = {
  node: {
    fs: 'empty',
  },
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html', 'favicon.svg', 'examples', 'index.css'])
  ],
  devServer: {
    // reference: https://webpack.js.org/configuration/dev-server/#devserverhttps
    https: {
      key: fs.readFileSync('localhost-key.pem'),
      cert: fs.readFileSync('localhost.pem'),
    },
    historyApiFallback: {
      index: 'index.html'
    },
  },
};
