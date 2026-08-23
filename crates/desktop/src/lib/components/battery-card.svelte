<script lang="ts">
  import Ear from "@lucide/svelte/icons/ear";
  import Zap from "@lucide/svelte/icons/zap";

  import BatteryBar from "./battery-bar.svelte";
  import { TONE_TEXT, batteryTone } from "$lib/battery";
  import { cn } from "$lib/utils";
  import type { Battery } from "$lib/ipc";

  interface Props {
    label: string;
    battery: Battery | null;
    inEar?: boolean;
    /** Shows a placeholder instead of a reading, for the moment before the first advertisement. */
    pending?: boolean;
    class?: string;
  }

  let {
    label,
    battery,
    inEar = false,
    pending = false,
    class: className
  }: Props = $props();

  const level = $derived(battery?.level ?? null);
  const charging = $derived(battery?.charging ?? false);
  const tone = $derived(batteryTone(level, charging));
</script>

<div
  class={cn(
    "bg-card border-hairline flex flex-col gap-3 rounded-xl border px-4 py-3.5",
    // An absent reading recedes rather than shouting an empty bar.
    level === null && !pending && "opacity-60",
    className
  )}
>
  <div class="flex items-center justify-between gap-2">
    <span class="text-muted-foreground text-[11px] font-medium tracking-widest uppercase">
      {label}
    </span>

    <span class="flex items-center gap-1">
      {#if inEar}
        <Ear class="text-muted-foreground size-3.5" aria-label="In ear" />
      {/if}
      {#if charging}
        <Zap
          class="text-battery-good size-3.5 animate-pulse fill-current"
          aria-label="Charging"
        />
      {/if}
    </span>
  </div>

  {#if pending}
    <div class="bg-muted h-8 w-16 animate-pulse rounded-lg"></div>
    <div class="bg-muted h-1.5 w-full animate-pulse rounded-full"></div>
  {:else}
    <div class="flex items-baseline gap-0.5">
      <span
        class={cn("text-3xl leading-none font-semibold tabular-nums", TONE_TEXT[tone])}
      >
        {level ?? "—"}
      </span>
      {#if level !== null}
        <span class="text-muted-foreground text-base font-medium">%</span>
      {/if}
    </div>

    <BatteryBar {level} {charging} />
  {/if}
</div>
