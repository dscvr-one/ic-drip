import { createApp } from "vue";
import App from "./App.vue";
import "virtual:windi.css";
import VueGtag from "vue-gtag-next";

/**
 * @dfinity/agent requires this. Can be removed once it's fixed
 */
window.global = window;

const app = createApp(App);

app.use(VueGtag, {
  property: {
    id: "G-KE88EVJHWW",
  },
});

app.mount("#app");
