<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import Pin from "@lucide/svelte/icons/pin";
  import PinOff from "@lucide/svelte/icons/pin-off";
  import X from "@lucide/svelte/icons/x";

  import { BatteryStat, DeviceArtwork, EmptyState } from "$lib/components";
  import { backend } from "$lib/ipc";
  import { artworkFor } from "$lib/models";
  import { bluetooth, devices } from "$lib/stores";
  import { cn } from "$lib/utils";
  import type { BatteryReading } from "$lib/battery";

  let hovered = $state(false);
  let pinned = $state(false);

  const artwork = $derived(artworkFor(devices.device?.model));
  const pending = $derived(devices.properties === null);

  /**
   * Readings for the buds group.
   *
   * The widget shows each bud separately rather than v0.1's single combined number — at a glance
   * "L 90 / R 20" is the useful information, and one averaged figure hides a bud about to die.
   */
  const budReadings = $derived.by<BatteryReading[]>(() => {
    const props = devices.properties;

    if (artwork.singleUnit) {
      return [
        {
          label: "Battery",
          short: "Battery",
          battery: props
            ? { level: devices.overallLevel ?? 0, charging: devices.isCharging }
            : null
        }
      ];
    }

    return [
      {
        label: "Left",
        short: "L",
        battery: props?.leftBattery ?? null,
        inEar: props?.leftInEar
      },
      {
        label: "Right",
        short: "R",
        battery: props?.rightBattery ?? null,
        inEar: props?.rightInEar
      }
    ];
  });

  onMount(async () => {
    pinned = await backend.isAlwaysOnTop();
  });

  async function togglePin() {
    pinned = !pinned;
    await backend.setAlwaysOnTop(pinned);
  }
</script>

<svelte:head>
  <title>winpods widget</title>
</svelte:head>

<!--
  `widget-root` is what tells `app.css` to keep the page background transparent, since this window
  is frameless and transparent and paints its own panel.

  `data-tauri-drag-region` makes the panel itself the window's drag handle.
-->
<div
  class="widget-root apple-glass relative flex h-screen w-screen flex-col overflow-hidden
         rounded-[14px] border border-white/[0.09] bg-black/60 text-white"
  data-tauri-drag-region
  onmouseenter={() => (hovered = true)}
  onmouseleave={() => (hovered = false)}
  role="presentation"
>
  <!-- Window controls fade in on hover so they never sit on top of the readings. -->
  <div
    class="absolute top-1.5 right-1.5 z-10 flex items-center gap-0.5 transition-opacity duration-150"
    class:opacity-0={!hovered}
    class:pointer-events-none={!hovered}
  >
    <button
      onclick={togglePin}
      aria-label={pinned ? "Unpin widget" : "Pin widget on top"}
      class={cn(
        "rounded-md p-1.5 transition-colors hover:bg-white/15",
        pinned ? "text-white" : "text-white/60 hover:text-white"
      )}
    >
      {#if pinned}
        <PinOff class="size-3" aria-hidden="true" />
      {:else}
        <Pin class="size-3" aria-hidden="true" />
      {/if}
    </button>
    <button
      onclick={() => void backend.hideWindow()}
      aria-label="Hide widget"
      class="rounded-md p-1.5 text-white/60 transition-colors hover:bg-red-500/80 hover:text-white"
    >
      <X class="size-3.5" aria-hidden="true" />
    </button>
  </div>

  {#if !bluetooth.isOn}
    <EmptyState
      icon="bluetooth-off"
      title="Bluetooth is off"
      description="Turn it on to see your device."
      onDark
      class="h-full"
    />
  {:else if !devices.device}
    <EmptyState
      icon="headphones"
      title="No device selected"
      description="Choose one from the dashboard."
      onDark
      class="h-full"
    />
  {:else if !devices.isConnected}
    <EmptyState
      icon="disconnected"
      title={devices.device.name}
      description="Disconnected"
      onDark
      class="h-full"
    />
  {:else}
    <div
      class="flex min-h-0 flex-1 items-center justify-center gap-5 px-5"
      data-tauri-drag-region
      in:fade={{ duration: 150 }}
    >
      <!-- Buds -->
      <div class="flex flex-col items-center gap-2">
        <DeviceArtwork model={devices.device.model} imgClass="h-10" />
        <div class="flex items-start gap-1.5">
          {#each budReadings as reading (reading.label)}
            <BatteryStat
              label={reading.short}
              battery={reading.battery}
              inEar={reading.inEar ?? false}
              {pending}
              class={artwork.singleUnit ? "w-16" : "w-12"}
            />
          {/each}
        </div>
      </div>

      <!-- Case, only for models that have one -->
      {#if artwork.case}
        <div class="h-14 w-px shrink-0 bg-white/10" aria-hidden="true"></div>

        <div class="flex flex-col items-center gap-2">
          <img
            src={artwork.case}
            alt="Charging case"
            class="h-10 w-auto object-contain"
          />
          <BatteryStat
            label="Case"
            battery={devices.properties?.caseBattery ?? null}
            {pending}
            class="w-12"
          />
        </div>
      {/if}
    </div>
  {/if}
</div>
