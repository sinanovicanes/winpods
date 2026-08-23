import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

/** Must match `devUrl` in src-tauri/tauri.conf.json. */
const DEV_PORT = 3000;

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],

  // Keep vite from clearing the terminal, otherwise it wipes out cargo's errors.
  clearScreen: false,

  server: {
    // Tauri loads this exact URL, so the port is fixed and vite must fail loudly rather than
    // silently move to another one. Keep it in step with `devUrl` in tauri.conf.json.
    port: DEV_PORT,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: DEV_PORT + 1 } : undefined,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  },

  // Tauri's webview is a known, modern target, so there is no reason to down-level.
  build: {
    target: "esnext",
    sourcemap: !!process.env.TAURI_ENV_DEBUG
  }
});
