type debounceFn = (...args: any[]) => any;

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
