const STORAGE_KEY = "winpods-theme";

export type ThemePreference = "light" | "dark" | "system";

/**
 * Light/dark handling.
 *
 * `app.html` applies the right class before first paint; this store keeps it in step afterwards
 * and follows the OS while the preference is "system", which is the default and what a Windows
 * app is expected to do.
 */
class ThemeStore {
  preference = $state<ThemePreference>("system");
  systemPrefersDark = $state(false);

  readonly isDark = $derived(
    this.preference === "dark" || (this.preference === "system" && this.systemPrefersDark)
  );

  #started = false;

  start() {
    if (this.#started || typeof window === "undefined") return;
    this.#started = true;

    const stored = safeRead();
    if (stored) {
      this.preference = stored;
    }

    const query = window.matchMedia("(prefers-color-scheme: dark)");
    this.systemPrefersDark = query.matches;
    query.addEventListener("change", event => {
      this.systemPrefersDark = event.matches;
    });

    // Keep the document in step with whatever `isDark` resolves to.
    $effect.root(() => {
      $effect(() => {
        const dark = this.isDark;
        document.documentElement.classList.toggle("dark", dark);
        document.documentElement.style.colorScheme = dark ? "dark" : "light";
      });
    });
  }

  set(preference: ThemePreference) {
    this.preference = preference;

    try {
      if (preference === "system") {
        localStorage.removeItem(STORAGE_KEY);
      } else {
        localStorage.setItem(STORAGE_KEY, preference);
      }
    } catch {
      // Private mode or storage disabled; the preference still applies for this session.
    }
  }
}

function safeRead(): ThemePreference | null {
  try {
    const value = localStorage.getItem(STORAGE_KEY);
    return value === "light" || value === "dark" ? value : null;
  } catch {
    return null;
  }
}

export const theme = new ThemeStore();
