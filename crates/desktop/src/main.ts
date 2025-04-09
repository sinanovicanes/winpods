import "./index.css";
import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import { createPlugin } from "@tauri-store/pinia";

const app = createApp(App);
const pinia = createPinia().use(createPlugin());

app.use(pinia);
app.mount("#app");
