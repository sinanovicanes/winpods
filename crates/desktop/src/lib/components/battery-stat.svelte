<script lang="ts">
  import Ear from "@lucide/svelte/icons/ear";
  import Zap from "@lucide/svelte/icons/zap";

  import BatteryBar from "./battery-bar.svelte";
  import { TONE_TEXT_ON_DARK, batteryTone } from "$lib/battery";
  import { cn } from "$lib/utils";
  import type { Battery } from "$lib/ipc";

  interface Props {
    /**
     * Names the reading for screen readers only.
     *
     * The widget draws no caption: a bare "L" under a percentage cost a line of vertical space to
     * repeat what the artwork beside it already says, and left is simply the one on the left.
     */
    label: string;
    battery: Battery | null;
    inEar?: boolean;
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

<div class={cn("flex w-14 flex-col items-center gap-1", className)}>
  <span class="sr-only">{label}</span>

  {#if pending}
    <div class="h-4 w-9 animate-pulse rounded bg-white/20"></div>
    <div class="h-1.5 w-full animate-pulse rounded-full bg-white/15"></div>
  {:else}
    <div class="flex items-center gap-0.5">
      <span
        class={cn(
          "text-sm leading-none font-semibold tabular-nums",
          TONE_TEXT_ON_DARK[tone]
        )}
      >
        {level === null ? "—" : `${level}%`}
      </span>
      {#if charging}
        <Zap
          class="size-3 animate-pulse fill-current text-white/90"
          aria-label="Charging"
        />
      {:else if inEar}
        <Ear class="size-3 text-white/60" aria-label="In ear" />
      {/if}
    </div>

    <BatteryBar {level} {charging} onDark />
  {/if}
</div>
