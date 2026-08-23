<script lang="ts">
  import BluetoothOff from "@lucide/svelte/icons/bluetooth-off";
  import Headphones from "@lucide/svelte/icons/headphones";
  import Unplug from "@lucide/svelte/icons/unplug";
  import TriangleAlert from "@lucide/svelte/icons/triangle-alert";
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";

  interface Props {
    title: string;
    description?: string;
    icon?: "bluetooth-off" | "headphones" | "disconnected" | "warning";
    /** Numbered guidance, for states the user has to fix outside the app. */
    steps?: string[];
    /** Renders for the widget's dark translucent panel. */
    onDark?: boolean;
    /** Optional action area, e.g. a refresh button or a device picker. */
    children?: Snippet;
    class?: string;
  }

  let {
    title,
    description,
    icon = "warning",
    steps,
    onDark = false,
    children,
    class: className
  }: Props = $props();

  const Icon = $derived(
    {
      "bluetooth-off": BluetoothOff,
      headphones: Headphones,
      disconnected: Unplug,
      warning: TriangleAlert
    }[icon]
  );
</script>

<div
  class={cn(
    "flex flex-col items-center justify-center gap-4 text-center",
    onDark ? "px-4" : "px-6",
    className
  )}
>
  <!-- The icon sits in a soft disc so an empty screen still has a focal point. -->
  <div
    class={cn(
      "flex items-center justify-center rounded-full",
      onDark ? "size-9 bg-white/10" : "bg-card border-hairline size-14 border shadow-sm"
    )}
  >
    <Icon
      class={cn(onDark ? "size-4 text-white/70" : "text-muted-foreground size-6")}
      aria-hidden="true"
    />
  </div>

  <div class="flex flex-col gap-1.5">
    <p
      class={cn(
        "font-semibold tracking-tight",
        onDark ? "text-[13px] text-white/90" : "text-base"
      )}
    >
      {title}
    </p>
    {#if description}
      <p
        class={cn(
          "max-w-xs leading-relaxed",
          onDark ? "text-[11px] text-white/50" : "text-muted-foreground text-[13px]"
        )}
      >
        {description}
      </p>
    {/if}
  </div>

  {#if steps && steps.length > 0 && !onDark}
    <ol class="text-muted-foreground flex max-w-xs flex-col gap-2 text-left text-[13px]">
      {#each steps as step, index (step)}
        <li class="flex gap-2.5">
          <span
            class="bg-foreground/[0.07] text-foreground flex size-5 shrink-0 items-center justify-center
                   rounded-full text-[11px] font-semibold tabular-nums"
          >
            {index + 1}
          </span>
          <span class="pt-0.5">{step}</span>
        </li>
      {/each}
    </ol>
  {/if}

  {#if children}
    <div class="mt-1 flex flex-col items-center gap-2">
      {@render children()}
    </div>
  {/if}
</div>
