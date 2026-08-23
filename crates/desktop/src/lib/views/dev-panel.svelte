<script lang="ts">
  /**
   * Dev-only controls for the mock backend.
   *
   * Only rendered when `backend.isLive` is false, which can never happen inside a Tauri window.
   * It exists so the states that are awkward to reach on real hardware -- bluetooth off, a
   * disconnected device, a critical battery, a bud that stopped reporting -- can be designed
   * against without Windows or AirPods.
   */
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import FlaskConical from "@lucide/svelte/icons/flask-conical";

  import { Button, Switch } from "$lib/components/ui";
  import { mockBackend } from "$lib/ipc";
  import { bluetooth, devices } from "$lib/stores";

  const dev = mockBackend.dev;

  let open = $state(false);

  const bluetoothOn = $derived(bluetooth.isOn);
  const connected = $derived(devices.isConnected);
  const charging = $derived(devices.isCharging);
  const inEar = $derived(devices.properties?.leftInEar ?? false);
  const caseReported = $derived(devices.properties?.caseBattery !== null);
</script>

<div class="fixed right-4 bottom-14 z-50 flex flex-col items-end gap-2">
  {#if open}
    <div
      class="bg-popover text-popover-foreground border-hairline w-72 rounded-xl border
             p-4 shadow-2xl"
    >
      <p class="text-muted-foreground mb-3 text-xs font-medium uppercase tracking-widest">
        Mock backend
      </p>

      <div class="flex flex-col gap-3 text-[13px]">
        <label class="flex items-center justify-between gap-3">
          Bluetooth on
          <Switch
            label="Bluetooth on"
            checked={bluetoothOn}
            onchange={value => dev.setAdapter(value ? "on" : "off")}
          />
        </label>

        <label class="flex items-center justify-between gap-3">
          Device connected
          <Switch
            label="Device connected"
            checked={connected}
            onchange={value => dev.setConnected(value)}
          />
        </label>

        <label class="flex items-center justify-between gap-3">
          Charging
          <Switch
            label="Charging"
            checked={charging}
            onchange={value => dev.setCharging(value)}
          />
        </label>

        <label class="flex items-center justify-between gap-3">
          In ear
          <Switch
            label="In ear"
            checked={inEar}
            onchange={value => dev.setInEar(value)}
          />
        </label>

        <label class="flex items-center justify-between gap-3">
          Case reports level
          <Switch
            label="Case reports level"
            checked={caseReported}
            onchange={value => dev.setCaseReported(value)}
          />
        </label>

        <div class="border-hairline flex flex-col gap-2 border-t pt-3">
          <span class="text-muted-foreground text-xs">Battery level</span>
          <div class="flex flex-wrap gap-1.5">
            {#each [0, 5, 15, 30, 60, 100] as level (level)}
              <Button variant="secondary" size="sm" onclick={() => dev.setBattery(level)}>
                {level}%
              </Button>
            {/each}
          </div>
        </div>

        <div class="border-hairline flex flex-col gap-2 border-t pt-3">
          <span class="text-muted-foreground text-xs">Bud stopped reporting</span>
          <div class="flex gap-1.5">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => dev.setBudReported("left", false)}
            >
              Drop left
            </Button>
            <Button
              variant="secondary"
              size="sm"
              onclick={() => dev.setBudReported("left", true)}
            >
              Restore
            </Button>
          </div>
        </div>

        <div class="border-hairline flex flex-col gap-2 border-t pt-3">
          <span class="text-muted-foreground text-xs">Device</span>
          <div class="flex flex-col gap-1.5">
            {#each dev.devices as device (device.address)}
              <Button
                variant="secondary"
                size="sm"
                onclick={() => void devices.select(device.address)}
              >
                {device.name}
              </Button>
            {/each}
            <Button
              variant="secondary"
              size="sm"
              onclick={() => void devices.disconnect()}
            >
              No device
            </Button>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <Button
    variant="secondary"
    size="sm"
    onclick={() => (open = !open)}
    aria-expanded={open}
  >
    <FlaskConical aria-hidden="true" />
    Mock
    <ChevronDown
      class="size-3 transition-transform duration-150 {open ? 'rotate-180' : ''}"
      aria-hidden="true"
    />
  </Button>
</div>
