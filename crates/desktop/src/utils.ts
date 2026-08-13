import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

type debounceFn = (...args: any[]) => any;

const INVOKE_RETRY_COUNT = 5;
const INVOKE_RETRY_DELAY = 200;

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

/**
 * Invokes a command, retrying a couple of times before giving up.
 *
 * Windows are created before the backend finishes its setup, so the first calls of a freshly
 * loaded webview may fail. Failing silently would leave the UI stuck with its default values
 * until the app is restarted.
 */
export async function invokeWithRetry<T>(
  command: string,
  args?: InvokeArgs,
  retries: number = INVOKE_RETRY_COUNT
): Promise<T> {
  for (let attempt = 0; ; attempt++) {
    try {
      return await invoke<T>(command, args);
    } catch (e) {
      if (attempt >= retries) {
        throw e;
      }

      console.warn(`[${command}] failed, retrying (${attempt + 1}/${retries}): ${e}`);
      await sleep(INVOKE_RETRY_DELAY * 2 ** attempt);
    }
  }
}

/**
 * Runs the callback whenever the window is brought back to the front, which is the moment the
 * user gets to see the state of the app again.
 */
export function onWindowShown(callback: () => void) {
  getCurrentWindow().onFocusChanged(({ payload: focused }) => {
    if (focused) {
      callback();
    }
  });
}

export function debounce<T extends debounceFn>(cb: T, wait: number) {
  let timeout: NodeJS.Timeout | undefined = undefined;

  return function (...args: Parameters<T>) {
    if (!!timeout) {
      clearTimeout(timeout);
      timeout = undefined;
    }

    timeout = setTimeout(() => {
      cb(...args);
      timeout = undefined;
    }, wait);
  };
}
