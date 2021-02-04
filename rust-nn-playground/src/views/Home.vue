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

@Options({
  components: {}
})
export default class Home extends Vue {
  wow = 0;
  rustnn: typeof RustnnType = inject("rustnn") as typeof RustnnType;
  network = this.rustnn.create_network(28 * 28, 10, 200, 0.1);

  created(): void {
    this.wow = this.rustnn.damn();
    console.log(this.network);
  }
}
</script>
