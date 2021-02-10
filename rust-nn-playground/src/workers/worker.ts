// eslint-disable-file
// eslint-disable-next-line @typescript-eslint/camelcase
import { MessageType, MnistMessage, TrainNetwork } from "@/workers/messages";
const ctx: Worker = self as any;

// Post data to parent thread
//ctx.postMessage("damn");

async function trainNetwork(params: TrainNetwork) {
  const rustnn = await import("rust-nn");
  ctx.postMessage({
    type: MessageType.TRAIN_UPDATE,
    value: { message: "Processing Dataset.", incrImage: 0 }
  });
  ctx.postMessage({
    type: MessageType.TRAIN_UPDATE,
    value: { message: "Building Network.", incrImage: 0 }
  });
  const network = new rustnn.Network(28 * 28, 10, params.hiddenNb, 0.1);
  let progressUpdateStepper = 0;

  for (const image of params.trainImages ?? []) {
    rustnn.train_for_mnist_dataset(network!, image.image, image.label);
    ++progressUpdateStepper;
    if (progressUpdateStepper < params.updateEveryNImage) {
      continue;
    }
    ctx.postMessage({
      type: MessageType.TRAIN_UPDATE,
      value: {
        message: "Training Network.",
        incrImage: params.updateEveryNImage
      }
    });
    progressUpdateStepper = 0;
  }
  ctx.postMessage({ type: MessageType.TRAIN_SUCCESS, value: { network } });
}

// Respond to message from parent thread
ctx.addEventListener("message", async event => {
  const msg = event.data as MnistMessage;
  if (msg.type == MessageType.TRAIN_NETWORK) {
    await trainNetwork(msg.value as TrainNetwork);
  }
});

