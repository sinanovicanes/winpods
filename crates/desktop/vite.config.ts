import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],

  // Keep vite from clearing the terminal, otherwise it wipes out cargo's errors.
  clearScreen: false,

  server: {
    // Tauri expects a fixed port and should fail loudly rather than silently move.
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
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
