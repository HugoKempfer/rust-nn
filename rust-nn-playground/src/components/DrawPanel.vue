<template>
  <div id="drawPanel" disabled>
    <h2 class="title is-3">Test Model</h2>
    <h3 class="subtitle is-6">Draw a digit at the center of the draw pad</h3>
    <div class="columns is-centered">
      <div class="column is-half">
        <SignaturePad :on-draw="predict" @clear="clear" v-if="network" />
        <h2 v-if="!network" class="is-2">
          You must train the network in order to test it!
        </h2>
        <p v-if="predicted != null">Predicted value = {{ predicted }}</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import SignaturePad from "@/components/SignaturePad.vue";
import { Network } from "rust-nn";

@Component({
  components: { SignaturePad }
})
export default class DrawPanel extends Vue {
  @Prop() network: Network | null = null;
  predicted: number | null = null;

  predict(base64Img: string) {
    if (this.network) {
      console.log(base64Img);
      const res = this.$rustnn.predict_from_raw_canvas(this.network, base64Img);

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
      this.predicted = highest;
    }
  }

  clear() {
    this.predicted = null;
  }
}
</script>

<style scoped></style>
