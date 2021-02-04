import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import * as Rustnn from "rust-nn";

//Hack for globally providing the type of the rustnn module
export type RustnnType = typeof Rustnn;

import("rust-nn").then(wasm => {
  const app = createApp(App).use(router);
  app.provide("rustnn", wasm);
  app.mount("#app");
});
