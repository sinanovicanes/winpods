import "./index.css";
import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import { createPlugin } from "@tauri-store/pinia";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { library } from "@fortawesome/fontawesome-svg-core";
import {
  faThumbTack,
  faThumbTackSlash,
  faXmark
} from "@fortawesome/free-solid-svg-icons";

library.add(faThumbTack, faThumbTackSlash, faXmark);

const app = createApp(App);
const pinia = createPinia().use(createPlugin());

app.component("font-awesome-icon", FontAwesomeIcon);
app.use(pinia);
app.mount("#app");
