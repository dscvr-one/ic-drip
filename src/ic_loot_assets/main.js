import { createApp } from "vue";
import App from "./App.vue";
import "virtual:windi.css";

/**
 * @dfinity/agent requires this. Can be removed once it's fixed
 */
window.global = window;

const app = createApp(App);


app.mount("#app");
