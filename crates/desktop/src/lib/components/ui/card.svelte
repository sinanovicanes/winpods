<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils";

  interface Props extends HTMLAttributes<HTMLDivElement> {
    /** Removes the padding so the card can hold its own full-width sections. */
    flush?: boolean;
  }

  let { flush = false, class: className, children, ...rest }: Props = $props();
</script>

<div
  class={cn(
    "bg-card text-card-foreground border-hairline rounded-xl border",
    flush || "p-6",
    className
  )}
  {...rest}
>
  {@render children?.()}
</div>
