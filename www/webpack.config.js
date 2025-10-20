const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');
const RemarkHTML = require('remark-html');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = (env, argv) => {
  const isDev = argv.mode === 'development' || env.WEBPACK_SERVE;
  const buildRoot = path.resolve(__dirname, "dist");

  return {
    mode: isDev ? "development" : "production",
    devtool: isDev ? "source-map" : false,
    entry: "./bootstrap.js",
    output: {
      filename: "bootstrap.js",
      path: buildRoot,
    },
    module: {
      rules: [
        // CSS rules
        {
          test: /\.css$/,
          use: [
            MiniCssExtractPlugin.loader,
            {
              loader: 'css-loader',
              options: { importLoaders: 1, sourceMap: isDev }
            }
          ],
        },
          // MARKDOWN rule
        {
          test: /\.md$/,
          use: [
            {
              loader: "html-loader",
              options: {
                minimize: false
              }
            },
            {
              loader: "remark-loader",
              options: {
                remarkOptions: {
                  plugins: [
                    RemarkHTML,
                  ],
                },
              },
            },
          ],
        },
      ],
    },
    experiments: {
      asyncWebAssembly: true,
    },

    plugins: [
      new HtmlWebpackPlugin({ template: 'index.html' }),
      new MiniCssExtractPlugin({ filename: '[name].[contenthash].css' })
    ],
  };
} 