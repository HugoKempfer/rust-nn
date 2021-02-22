<template>
  <div id="drawPanel">
    <h2 class="title is-3">Test Model</h2>
    <div class="columns is-centered">
      <div class="column is-half">
        <SignaturePad :on-draw="predict" @clear="clear" />
        <p v-if="predicted">Predicted value = {{ predicted }}</p>
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
