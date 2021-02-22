// eslint-disable-file
// eslint-disable-next-line @typescript-eslint/camelcase
import { MessageType, MnistMessage, TrainNetwork } from "@/workers/messages";

const ctx: Worker = self as any;

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

  // Test network
  ctx.postMessage({
    type: MessageType.TRAIN_UPDATE,
    value: { message: "Testing network", incrImage: 0 }
  });
  let correct = 0;
  let error = 0;

  for (const image of params.testImages) {
    const res = rustnn.predict_for_mnist_dataset(network, image.image);
    let highest = 0;
    let highestValue = 0;
    let it = 0;
    for (const val of res) {
      if (val > highestValue) {
        highestValue = val;
        highest = it;
      }
      ++it;
    }
    if (highest == image.label.findIndex(value => value == 1)) {
      ++correct;
    } else {
      ++error;
    }
  }
  ctx.postMessage({
    type: MessageType.TRAIN_SUCCESS,
    value: {
      network: network.to_ron(),
      correctPredictNb: correct,
      errorPredictNb: error
    }
  });
}

// Respond to message from parent thread
ctx.addEventListener("message", async event => {
  const msg = event.data as MnistMessage;
  if (msg.type == MessageType.TRAIN_NETWORK) {
    await trainNetwork(msg.value as TrainNetwork);
  }
});
