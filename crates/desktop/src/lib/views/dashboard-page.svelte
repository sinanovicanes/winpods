<script lang="ts">
  import RefreshCw from "@lucide/svelte/icons/refresh-cw";

  import { BatteryIndicator, DeviceArtwork, StatusMessage } from "$lib/components";
  import { Button, Card, Select, Switch } from "$lib/components/ui";
  import { artworkFor } from "$lib/models";
  import { devices, settings } from "$lib/stores";

  const artwork = $derived(artworkFor(devices.device?.model));

  const deviceOptions = $derived(
    devices.available.map(device => ({ value: device.address, label: device.name }))
  );

  /**
   * Battery rows to show.
   *
   * Over-ear models are one unit with one battery, so they get a single row -- a left/right
   * split would invent a distinction the hardware does not have. Paired models always show both
   * buds (an absent reading renders as a dash) and a case row only if they have a case.
   */
  const rows = $derived.by(() => {
    const props = devices.properties;

    if (artwork.singleUnit) {
      const battery = props
        ? { level: devices.overallLevel, charging: devices.isCharging }
        : null;

      return [{ label: "Battery", battery, show: true }];
    }

    return [
      { label: "Left", battery: props?.leftBattery ?? null, show: true },
      { label: "Right", battery: props?.rightBattery ?? null, show: true },
      {
        label: "Case",
        battery: props?.caseBattery ?? null,
        show: artwork.case !== undefined
      }
    ];
  });
</script>

{#if devices.loading}
  <div class="flex min-h-96 items-center justify-center">
    <div class="bg-muted h-24 w-full max-w-md animate-pulse rounded-xl"></div>
  </div>
{:else if devices.device}
  {@const device = devices.device}

  <div class="mx-auto flex max-w-2xl flex-col gap-4">
    <Card class="flex flex-col gap-6">
      <div class="flex items-start justify-between gap-6">
        <div class="flex min-w-0 flex-col gap-4">
          <div class="min-w-0">
            <h1 class="truncate text-xl font-semibold tracking-tight">{device.name}</h1>
            <p class="text-muted-foreground mt-0.5 text-[13px]">
              {artwork.name}
              {#if !devices.isConnected}
                &middot; <span class="text-destructive">Disconnected</span>
              {/if}
            </p>
          </div>

          <dl class="flex flex-col gap-2.5">
            {#each rows as row (row.label)}
              {#if row.show}
                <div class="flex items-center gap-4">
                  <dt class="text-muted-foreground w-16 text-[13px]">{row.label}</dt>
                  <dd>
                    {#if devices.properties}
                      <BatteryIndicator
                        level={row.battery?.level ?? null}
                        charging={row.battery?.charging ?? false}
                      />
                    {:else}
                      <span class="bg-muted block h-3 w-16 animate-pulse rounded-full"
                      ></span>
                    {/if}
                  </dd>
                </div>
              {/if}
            {/each}
          </dl>
        </div>

        <DeviceArtwork
          model={device.model}
          variant="hero"
          class="h-32 w-40 shrink-0"
          imgClass="h-32"
        />
      </div>
    </Card>

    <Card class="flex items-center justify-between gap-6">
      <div class="min-w-0">
        <p class="text-[13px] font-medium">Automatic ear detection</p>
        <p class="text-muted-foreground mt-0.5 text-xs">
          {artwork.singleUnit
            ? "Pauses audio when you take them off and resumes when you put them back on."
            : "Pauses audio when you take a bud out and resumes when you put it back in."}
        </p>
      </div>
      <Switch
        label="Automatic ear detection"
        checked={settings.current.earDetection}
        onchange={value => void settings.update({ earDetection: value })}
      />
    </Card>

    <div class="flex justify-end">
      <Button variant="destructive" onclick={() => void devices.disconnect()}>
        Forget this device
      </Button>
    </div>
  </div>
{:else}
  <div class="mx-auto flex max-w-2xl flex-col gap-4">
    <Card class="flex flex-col gap-5">
      <div class="flex items-start justify-between gap-4">
        <div>
          <h1 class="text-xl font-semibold tracking-tight">Choose a device</h1>
          <p class="text-muted-foreground mt-0.5 text-[13px]">
            Pick one of your connected bluetooth devices to monitor.
          </p>
        </div>
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
      </div>

      {#if deviceOptions.length > 0}
        <Select
          label="Device"
          value={null}
          placeholder="Select a device"
          options={deviceOptions}
          onchange={address => void devices.select(address)}
          class="w-full [&>select]:w-full"
        />
      {:else}
        <StatusMessage
          icon="headphones"
          title="No connected devices"
          message="Connect your AirPods in Windows bluetooth settings, then refresh."
          class="py-8"
        />
      {/if}
    </Card>
  </div>
{/if}
