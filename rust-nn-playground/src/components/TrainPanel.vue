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
    this.dataset = await buildMnist(10000, 100);
  }

  async doTrain() {
    this.progressBarValue = 0;
    this.isTraining = true;
    await this.refreshDataset();
    this.trainStepText = "Building Neural Network";
    this.network = new this.$rustnn.Network(28 * 28, 10, this.hiddenNb, 0.1);
    this.trainStepText = "Training Neural Network";

    let progressUpdateStepper = 0;

    for (const image of this.dataset?.trainImages ?? []) {
      this.$rustnn.train_for_mnist_dataset(
        this.network!,
        image.image,
        image.label
      );
      ++progressUpdateStepper;
      if (progressUpdateStepper >= 500) {
        this.progressBarValue += 500;
        progressUpdateStepper = 0;
      }
    }
    if (this.trainCallback) {
      this.trainCallback(this.network);
    }
    this.isTraining = false;
  }
}
</script>

<style scoped></style>
