import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * Merges class names, letting a caller's utility win over a component's default.
 *
 * The standard shadcn helper: `clsx` flattens conditionals, `tailwind-merge` resolves
 * conflicts so `class="p-8"` on a component with `p-4` yields `p-8` rather than both.
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
