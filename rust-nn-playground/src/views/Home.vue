<template>
  <div class="home">
    <h1 class="title is-1">Rust NN Playground</h1>
    <h2 class="subtitle is-4">
      Play and visualize AI written in Rust and running under WASM
    </h2>
    <section class="section is-large">
      <div class="columns">
        <div class="column">
          <TrainPanel :train-callback="trainCallback" />
        </div>
        <div class="column">
          <DrawPanel />
        </div>
      </div>
    </section>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import TrainPanel from "@/components/TrainPanel.vue";
import DrawPanel from "@/components/DrawPanel.vue";
import { Network } from "rust-nn";

@Component({
  components: {
    DrawPanel,
    TrainPanel
  }
})
export default class Home extends Vue {
  correct = 0;
  error = 0;
  network: Network | null = null;

  async created() {
    //
  }
  trainCallback(newModel: Network) {
    this.network = newModel;
  }
}
</script>
