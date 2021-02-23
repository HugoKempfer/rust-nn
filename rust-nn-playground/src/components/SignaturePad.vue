<template>
  <div id="signaturePad">
    <canvas height="112px" width="112px" id="signaturePadCanva" />
    <b-button @click="clear">Clear</b-button>
  </div>
</template>

<script lang="ts">
import { Component, Emit, Prop, Vue } from "vue-property-decorator";
import SignaturePad from "signature_pad";

@Component({})
export default class SignaturePadComponent extends Vue {
  signaturePad: SignaturePad | undefined = undefined;
  @Prop() onDraw: ((imgBase64: string) => void) | undefined;

  mounted() {
    this.setupDrawPad();
  }

  setupDrawPad() {
    const canvas = document.querySelector("canvas");
    if (canvas) {
      this.signaturePad = new SignaturePad(canvas, {
        backgroundColor: "white",
        onEnd: () => {
          console.log(this.signaturePad?.toDataURL());
          this.onDraw?.((this.signaturePad?.toDataURL() ?? "").split(",")[1]);
        }
      });
    }
  }

  @Emit()
  clear() {
    this.setupDrawPad();
  }
}
</script>

<style>
canvas {
  border: 1px solid black;
}
</style>
