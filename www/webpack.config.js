const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');
const RemarkHTML = require('remark-html');

module.exports = env => {
  const buildRoot = path.resolve(__dirname, "dist");
  const srcRoot = path.resolve(__dirname, "src"); 
  const isDev = true;//env === "dev";
  const sourceMap = isDev;
  const minimize = !isDev; //we only minimize in production env

  return {
    mode: isDev ? "development" : "production",
    devtool: sourceMap ? "source-map" : false,
    entry: "./index.js",
    output: {
      filename: "index.js",
      path: buildRoot,
    },
    module: {
      rules: [
        // CSS rules
        {
          test: /\.css$/,
          use: [
            'style-loader',
            {
              loader: 'css-loader',
              options: {
                import: false,
                modules: true
              }
            }
          ],
          include: /\.module\.css$/
        },
        {
          test: /\.css$/,
          use: [
            MiniCssExtractPlugin.loader,
            'css-loader'
          ],
          exclude: /\.module\.css$/
        },
          // MARKDOWN rule
        {
          test: /\.md$/,
          use: [
            {
              loader: "html-loader",
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
      new CopyWebpackPlugin({ patterns: [{ from: "index.html" }] }),
      new MiniCssExtractPlugin({ filename: "styles.css" })
    ],
  };
} 