import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
export default {
  preprocess: vitePreprocess(),
  kit: {
    // Tauri serves plain files from disk, so every route is prerendered to HTML.
    // No SPA fallback: an accidentally non-prerenderable route should fail the build
    // rather than become a blank window at runtime.
    adapter: adapter(),
    alias: { "@": "src" },
    typescript: {
      config: config => {
        config.include.push("../*.ts", "../*.js");
        return config;
      }
    }
  }
};
