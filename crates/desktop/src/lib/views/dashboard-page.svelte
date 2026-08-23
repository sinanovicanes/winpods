<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import RefreshCw from "@lucide/svelte/icons/refresh-cw";

  import { BatteryCard, DeviceArtwork, EmptyState } from "$lib/components";
  import { Button, Select, Switch } from "$lib/components/ui";
  import { artworkFor } from "$lib/models";
  import { devices, settings } from "$lib/stores";
  import { cn } from "$lib/utils";
  import type { BatteryReading } from "$lib/battery";

  const artwork = $derived(artworkFor(devices.device?.model));

  /** True until the first advertisement lands, so cards show a placeholder not a dash. */
  const pending = $derived(devices.properties === null && devices.isConnected);

  /**
   * Battery readings to show.
   *
   * Over-ear models are one unit with one battery, so they get a single card — a left/right split
   * would invent a distinction the hardware does not have.
   */
  const readings = $derived.by<BatteryReading[]>(() => {
    const props = devices.properties;

    if (artwork.singleUnit) {
      return [{ label: "Battery", battery: devices.overallBattery }];
    }

    const readings: BatteryReading[] = [
      {
        label: "Left",
        battery: props?.leftBattery ?? null,
        inEar: props?.leftInEar
      },
      {
        label: "Right",
        battery: props?.rightBattery ?? null,
        inEar: props?.rightInEar
      }
    ];

    if (artwork.case) {
      readings.push({
        label: "Case",
        battery: props?.caseBattery ?? null
      });
    }

    return readings;
  });

  // Tailwind needs static class names, so the column count is mapped rather than interpolated.
  const gridClass = $derived(
    {
      1: "grid-cols-1 max-w-[220px]",
      2: "grid-cols-2 max-w-md",
      3: "grid-cols-3 max-w-xl"
    }[readings.length] ?? "grid-cols-3 max-w-xl"
  );

  const deviceOptions = $derived(
    devices.available.map(device => ({ value: device.address, label: device.name }))
  );

  /** A short summary of where the buds are, shown under the device name. */
  const wearing = $derived.by(() => {
    const props = devices.properties;
    if (!props || artwork.singleUnit) return null;
    if (props.leftInEar && props.rightInEar) return "Both in ear";
    if (props.leftInEar) return "Left in ear";
    if (props.rightInEar) return "Right in ear";
    return null;
  });
</script>

{#if devices.loading}
  <!-- First paint before the snapshot arrives. Mirrors the real layout so nothing jumps. -->
  <div class="flex min-h-full flex-col items-center justify-center gap-6">
    <div class="bg-muted size-32 animate-pulse rounded-full"></div>
    <div class="flex flex-col items-center gap-2">
      <div class="bg-muted h-6 w-48 animate-pulse rounded-lg"></div>
      <div class="bg-muted h-4 w-28 animate-pulse rounded-lg"></div>
    </div>
    <div class="grid w-full max-w-xl grid-cols-3 gap-3">
      {#each [0, 1, 2] as key (key)}
        <div class="bg-muted h-28 animate-pulse rounded-xl"></div>
      {/each}
    </div>
  </div>
{:else if devices.device}
  {@const device = devices.device}

  <div
    class="flex min-h-full flex-col items-center justify-center gap-6"
    in:fade={{ duration: 200 }}
  >
    <!-- Hero: the product itself is the anchor of the screen. -->
    <div class="flex flex-col items-center gap-4">
      <DeviceArtwork
        model={device.model}
        variant="hero"
        imgClass={cn(
          "h-32 transition-all duration-500",
          // A disconnected device is visibly inert rather than just labelled.
          !devices.isConnected && "grayscale opacity-40"
        )}
      />

      <div class="flex flex-col items-center gap-2 text-center">
        <h1 class="max-w-md truncate text-2xl font-semibold tracking-tight">
          {device.name}
        </h1>

        <div class="text-muted-foreground flex items-center gap-2 text-[13px]">
          <span>{artwork.name}</span>
          <span aria-hidden="true">·</span>
          {#if devices.isConnected}
            <span class="flex items-center gap-1.5">
              <span class="bg-battery-good size-1.5 rounded-full" aria-hidden="true"
              ></span>
              Connected
            </span>
          {:else}
            <span class="text-destructive flex items-center gap-1.5">
              <span class="bg-destructive size-1.5 rounded-full" aria-hidden="true"
              ></span>
              Disconnected
            </span>
          {/if}
        </div>

        {#if wearing}
          <p class="text-muted-foreground text-xs" in:fade={{ duration: 150 }}>
            {wearing}
          </p>
        {/if}
      </div>
    </div>

    <!-- Batteries: the reason the app exists, so they get the most weight. -->
    {#if devices.isConnected}
      <div class={cn("grid w-full gap-3", gridClass)} in:fly={{ y: 8, duration: 250 }}>
        {#each readings as reading (reading.label)}
          <BatteryCard
            label={reading.label}
            battery={reading.battery}
            inEar={reading.inEar ?? false}
            {pending}
          />
        {/each}
      </div>
    {:else}
      <p class="text-muted-foreground max-w-xs text-center text-[13px] leading-relaxed">
        Reconnect the device in Windows bluetooth settings to see its battery levels
        again.
      </p>
    {/if}

    <!-- Secondary controls, deliberately quieter than the readings above. -->
    <div class="flex w-full max-w-xl flex-col items-center gap-3">
      <div
        class="bg-card border-hairline flex w-full items-center justify-between gap-6 rounded-xl
               border px-4 py-3"
      >
        <div class="min-w-0">
          <p class="text-[13px] font-medium">Automatic ear detection</p>
          <p class="text-muted-foreground mt-0.5 text-xs">
            {artwork.singleUnit
              ? "Pauses audio when you take them off."
              : "Pauses audio when you take a bud out."}
          </p>
        </div>
        <Switch
          label="Automatic ear detection"
          checked={settings.current.earDetection}
          onchange={value => void settings.update({ earDetection: value })}
        />
      </div>

      <!-- Muted until hovered: a destructive action should be reachable, not inviting. -->
      <Button
        variant="ghost"
        size="sm"
        class="text-muted-foreground hover:text-destructive hover:bg-destructive/10"
        onclick={() => void devices.disconnect()}
      >
        Forget this device
      </Button>
    </div>
  </div>
{:else}
  <div class="flex min-h-full items-center justify-center" in:fade={{ duration: 200 }}>
    {#if deviceOptions.length > 0}
      <EmptyState
        icon="headphones"
        title="Choose a device"
        description="Pick one of your connected bluetooth devices to start monitoring it."
      >
        <Select
          label="Device"
          value={null}
          placeholder="Select a device"
          options={deviceOptions}
          onchange={address => void devices.select(address)}
          class="w-64 [&>select]:w-full"
        />
        <Button
          variant="ghost"
          size="sm"
          loading={devices.refreshing}
          onclick={() => void devices.refreshAvailable()}
        >
          {#if !devices.refreshing}
            <RefreshCw aria-hidden="true" />
          {/if}
          Refresh
        </Button>
      </EmptyState>
    {:else}
      <EmptyState
        icon="headphones"
        title="No devices connected"
        description="winpods reads battery levels from a device Windows is already connected to."
        steps={[
          "Open your AirPods case, or put them on.",
          "Connect them in Windows bluetooth settings.",
          "Come back here and refresh."
        ]}
      >
        <Button
          variant="secondary"
          size="sm"
          loading={devices.refreshing}
          onclick={() => void devices.refreshAvailable()}
        >
          {#if !devices.refreshing}
            <RefreshCw aria-hidden="true" />
          {/if}
          Refresh
        </Button>
      </EmptyState>
    {/if}
  </div>
{/if}
