<script lang="ts">
  import { cn } from "$lib/utils";

  let { class: className }: { class?: string } = $props();
</script>

<svg
  class={cn("size-4 animate-spin", className)}
  viewBox="0 0 24 24"
  fill="none"
  aria-hidden="true"
>
  <circle
    cx="12"
    cy="12"
    r="10"
    stroke="currentColor"
    stroke-width="2"
    class="opacity-25"
  />
  <path
    fill="currentColor"
    class="opacity-90"
    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
  />
</svg>
