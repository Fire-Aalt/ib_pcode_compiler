const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');
const RemarkHTML = require('remark-html');

module.exports = (env, argv) => {
  const isDev = argv.mode === 'development' || env.WEBPACK_SERVE;
  const buildRoot = path.resolve(__dirname, "dist");
  const srcRoot = path.resolve(__dirname, "src"); 
  const sourceMap = isDev;

  return {
    mode: isDev ? "development" : "production",
    devtool: sourceMap ? "source-map" : false,
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
      new CopyWebpackPlugin({ patterns: [{ from: "index.html" }] }),
      new MiniCssExtractPlugin({ filename: "styles.css" })
    ],
  };
} 