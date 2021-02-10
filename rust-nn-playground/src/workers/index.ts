export const worker = new Worker("./worker.ts", {
  type: "module",
  name: "MnistWorker"
});
