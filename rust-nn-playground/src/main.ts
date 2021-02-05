import App from "./App.vue";
import * as Rustnn from "rust-nn";
import Buefy from "buefy";
import Vue from "vue";
import _Vue from "vue";
import "buefy/dist/buefy.css";

Vue.config.productionTip = false;

//Hack for globally providing the type of the rustnn module
export type RustnnType = typeof Rustnn;

import("rust-nn").then(wasm => {
  const app = new Vue({ render: h => h(App) });
  const rustnn = {
    // eslint-disable-next-line
    install(Vue: typeof _Vue, options: any) {
      Vue.prototype.$rustnn = wasm;
    }
  };
  Vue.use(rustnn);
  Vue.use(Buefy);
  app.$mount("#app");
});

declare module "vue/types/vue" {
  interface Vue {
    $rustnn: RustnnType;
  }
}
