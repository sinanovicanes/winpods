<script lang="ts">
  import BluetoothOff from "@lucide/svelte/icons/bluetooth-off";
  import Headphones from "@lucide/svelte/icons/headphones";
  import TriangleAlert from "@lucide/svelte/icons/triangle-alert";
  import { cn } from "$lib/utils";

  interface Props {
    title: string;
    message?: string;
    icon?: "bluetooth-off" | "headphones" | "warning";
    /** Renders for the widget's dark translucent panel. */
    onDark?: boolean;
    class?: string;
  }

  let {
    title,
    message,
    icon = "warning",
    onDark = false,
    class: className
  }: Props = $props();

  const Icon = $derived(
    { "bluetooth-off": BluetoothOff, headphones: Headphones, warning: TriangleAlert }[
      icon
    ]
  );
</script>

<div
  class={cn(
    "flex h-full w-full flex-col items-center justify-center gap-1.5 px-6 text-center",
    className
  )}
>
  <Icon
    class={cn("mb-1 size-6", onDark ? "text-white/60" : "text-muted-foreground")}
    aria-hidden="true"
  />
  <p class={cn("text-sm font-medium", onDark ? "text-white/90" : "text-foreground")}>
    {title}
  </p>
  {#if message}
    <p class={cn("text-xs", onDark ? "text-white/50" : "text-muted-foreground")}>
      {message}
    </p>
  {/if}
</div>
