<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    {{ wow }}
    err =>{{ error }}
    correct =>{{ correct }}
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import { inject } from "vue";
import { RustnnType } from "@/main";
import { buildMnist } from "@/models/MnistDatasetModel";

@Options({
  components: {}
})
export default class Home extends Vue {
  wow = 0;
  correct = 0;
  error = 0;
  rustnn: RustnnType = inject("rustnn") as RustnnType;
  network = new this.rustnn.Network(28 * 28, 10, 200, 0.1);

  async created() {
    console.log(this.network);
    console.log("Before loading dataset");
    const dataset = await buildMnist(10000, 100);
    console.log(dataset);
    for (const image of dataset.trainImages) {
      this.rustnn.train_for_mnist_dataset(
        this.network,
        image.image,
        image.label
      );
      this.wow++;
    }
    console.log(this.network);
    for (const image of dataset.testImages) {
      const res = this.rustnn.predict_for_mnist_dataset(
        this.network,
        image.image
      );
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
        ++this.correct;
      } else {
        ++this.error;
      }
    }
  }
}
</script>
