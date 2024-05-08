
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

// change this to run different examples
module.exports = env => {
  return {
    entry: [`./${env.name}/app.js`],
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'app.js',
    },
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          use: 'ts-loader',
          exclude: /node_modules/,
        },
      ],
    },
    resolve: {
      extensions: ['.tsx', '.ts', '.js'],
    },

    devServer: {
      client: {
        //prevent control flow error from showing on screen
        overlay: false
      }
    },

    plugins: [
      new HtmlWebpackPlugin({
        template: `./${env.name}/index.html`
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "./engine/")
      }),
      // Have this example work in Edge which doesn't ship `TextEncoder` or
      // `TextDecoder` at this time.
      new webpack.ProvidePlugin({
        TextDecoder: ['text-encoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder']
      })
    ],
    mode: 'development',
    experiments: {
      asyncWebAssembly: true
    }
  }
};
