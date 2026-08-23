import { mockBackend } from "./mock";
import { tauriBackend } from "./tauri";
import type { Backend } from "./backend";

/**
 * Whether the UI is running inside a Tauri window.
 *
 * Tauri injects `__TAURI_INTERNALS__` before any app code runs, so this is settled by the time
 * the first module evaluates.
 */
function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/**
 * The backend the UI talks to.
 *
 * Inside a Tauri window this is the real Rust app. Anywhere else -- `bun run dev` in a browser,
 * on any OS -- it is the mock, so the UI can be built and designed without Windows.
 */
export const backend: Backend = isTauri() ? tauriBackend : mockBackend;

export { mockBackend };
export { Events } from "./events";
export type { Backend, Unlisten } from "./backend";
export type * from "./types";
