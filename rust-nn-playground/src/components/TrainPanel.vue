<template>
  <div id="TrainPanel">
    <h2 class="title is-3">Model Setup</h2>
    <div class="block">
      <b-field label="Number of hidden neurons">
        <b-slider
          indicator
          :min="10"
          :max="500"
          v-model="hiddenNb"
          :tooltip="false"
        />
      </b-field>
      <b-field label="Train dataset size">
        <b-slider
          indicator
          :min="1"
          :max="60000"
          v-model="trainSize"
          :tooltip="false"
        />
      </b-field>
      <b-field label="Test dataset size">
        <b-slider
          indicator
          :min="1"
          :max="1000"
          v-model="testSize"
          :tooltip="false"
        />
      </b-field>
      <button
        class="button is-two-fifths"
        @click="doTrain"
        :disabled="isTraining"
      >
        Train model
      </button>
    </div>
    <div class="block" v-if="isTraining">
      <p>{{ trainStepText }}</p>
      <b-progress
        :value="progressBarValue ? progressBarValue : undefined"
        :max="trainSize"
      >
      </b-progress>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import { Network } from "rust-nn";
import { buildMnist } from "@/models/MnistDatasetModel";
import { MnistImage } from "@/models/MnistDatasetModel";
import { worker } from "@/workers";
import {
  MessageType,
  MnistMessage,
  TrainNetwork,
  TrainSuccess,
  TrainUpdate
} from "@/workers/messages";

@Component({})
export default class TrainPanel extends Vue {
  @Prop() trainCallback: ((net: Network) => void) | undefined;
  hiddenNb = 100;
  trainSize = 30000;
  testSize = 1000;
  isTraining = false;
  trainStepText = "";
  progressBarValue = 0;
  dataset: {
    trainImages: MnistImage[];
    testImages: MnistImage[];
  } | null = null;
  network: Network | null = null;

  async refreshDataset() {
    if (
      this.dataset != null &&
      this.dataset.trainImages.length == this.trainSize &&
      this.dataset.testImages.length == this.testSize
    ) {
      return;
    }
    this.trainStepText = "Processing dataset";
    this.dataset = await buildMnist(this.trainSize, this.testSize);
  }

  async doTrain() {
    this.progressBarValue = 0;
    this.isTraining = true;
    await this.refreshDataset();
    // eslint-disable-next-line no-undef
    const trainParams: TrainNetwork = {
      hiddenNb: this.hiddenNb,
      learningRate: 0.1,
      trainImages: this.dataset?.trainImages ?? [],
      testImages: this.dataset?.testImages ?? [],
      updateEveryNImage: 50
    };
    worker.postMessage({
      // eslint-disable-next-line no-undef
      type: MessageType.TRAIN_NETWORK,
      value: trainParams
    });
    // eslint-disable-next-line no-undef
    worker.onmessage = ev => {
      const msg = ev.data as MnistMessage;
      switch (msg.type) {
        // eslint-disable-next-line no-undef
        case MessageType.TRAIN_UPDATE:
          this.trainStepText = (msg.value as TrainUpdate).message;
          this.progressBarValue += (msg.value as TrainUpdate).incrImage;
          break;
        // eslint-disable-next-line no-undef
        case MessageType.TRAIN_SUCCESS:
          this.network = (msg.value as TrainSuccess).network as Network;
          if (this.trainCallback) {
            this.trainCallback(this.network as Network);
          }
          this.isTraining = false;
          this.progressBarValue = 0;
          this.trainStepText = "";
      }
    };
  }
}
</script>

<style scoped></style>
