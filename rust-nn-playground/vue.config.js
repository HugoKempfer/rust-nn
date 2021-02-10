// eslint-disable-next-line @typescript-eslint/no-var-requires
const WorkerPlugin = require("worker-plugin");
module.exports = {
  configureWebpack: {
    output: {
      globalObject: "this"
    },
    plugins: [new WorkerPlugin()]
  }
};
