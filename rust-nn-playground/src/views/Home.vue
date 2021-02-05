<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    {{ wow }}
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
  rustnn: RustnnType = inject("rustnn") as RustnnType;
  network = this.rustnn.create_network(28 * 28, 10, 200, 0.1);

  async created() {
    this.wow = this.rustnn.damn();
    console.log(this.network);
    console.log("Before loading dataset");
    const dataset = await buildMnist(1000, 500);
    console.log(dataset);
  }
}
</script>
