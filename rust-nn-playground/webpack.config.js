module.exports = {
  module: {
    rules: [
      {
        test: /\.wasm$/,
        loaders: ["wasm-loader"]
      }
    ]
  }
};
