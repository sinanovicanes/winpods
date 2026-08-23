<script lang="ts">
  import { onMount } from "svelte";
  import Pin from "@lucide/svelte/icons/pin";
  import PinOff from "@lucide/svelte/icons/pin-off";
  import X from "@lucide/svelte/icons/x";

  import { BatteryIndicator, DeviceArtwork, StatusMessage } from "$lib/components";
  import { backend } from "$lib/ipc";
  import { artworkFor } from "$lib/models";
  import { bluetooth, devices } from "$lib/stores";

  let hovered = $state(false);
  let pinned = $state(false);

  const artwork = $derived(artworkFor(devices.device?.model));

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
  `widget-root` is what tells `app.css` to keep the page background transparent, since this
  window is frameless and transparent and paints its own panel.

  `data-tauri-drag-region` makes the panel itself the window's drag handle.
-->
<div
  class="widget-root apple-glass relative flex h-screen w-screen flex-col overflow-hidden
         rounded-xl border border-white/10 bg-black/55 text-white"
  data-tauri-drag-region
  onmouseenter={() => (hovered = true)}
  onmouseleave={() => (hovered = false)}
  role="presentation"
>
  <!-- Window controls fade in on hover so they never obscure the readings. -->
  <div
    class="absolute top-1 right-1 z-10 flex items-center gap-0.5 transition-opacity duration-150"
    class:opacity-0={!hovered}
    class:pointer-events-none={!hovered}
  >
    <button
      onclick={togglePin}
      aria-label={pinned ? "Unpin widget" : "Pin widget on top"}
      class="rounded-md p-1.5 text-white/70 transition-colors hover:bg-white/15 hover:text-white"
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
      class="rounded-md p-1.5 text-white/70 transition-colors hover:bg-red-500/70 hover:text-white"
    >
      <X class="size-3.5" aria-hidden="true" />
    </button>
  </div>

  {#if !bluetooth.isOn}
    <StatusMessage
      icon="bluetooth-off"
      title="Bluetooth is off"
      message="Turn it on to see your device."
      onDark
    />
  {:else if !devices.device}
    <StatusMessage
      icon="headphones"
      title="No device selected"
      message="Pick one from the dashboard."
      onDark
    />
  {:else if !devices.isConnected}
    <StatusMessage
      icon="headphones"
      title={devices.device.name}
      message="Disconnected"
      onDark
    />
  {:else}
    <div
      class="flex min-h-0 flex-1 items-center justify-evenly gap-4 px-6"
      data-tauri-drag-region
    >
      <!-- Buds -->
      <div class="flex flex-col items-center gap-1.5">
        <DeviceArtwork model={devices.device.model} imgClass="h-11" />
        {#if devices.properties}
          <BatteryIndicator
            level={devices.overallLevel}
            charging={devices.isCharging}
            onDark
          />
        {:else}
          <span class="block h-3 w-14 animate-pulse rounded-full bg-white/20"></span>
        {/if}
      </div>

      <!-- Case, only for models that have one -->
      {#if artwork.case}
        <div class="flex flex-col items-center gap-1.5">
          <img
            src={artwork.case}
            alt="Charging case"
            class="h-11 w-auto object-contain"
          />
          {#if devices.properties}
            <BatteryIndicator
              level={devices.properties.caseBattery?.level ?? null}
              charging={devices.properties.caseBattery?.charging ?? false}
              onDark
            />
          {:else}
            <span class="block h-3 w-14 animate-pulse rounded-full bg-white/20"></span>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</div>
