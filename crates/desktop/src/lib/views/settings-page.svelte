<script lang="ts">
  import { Card, Select, Switch } from "$lib/components/ui";
  import { settings, theme } from "$lib/stores";
  import type { ThemePreference } from "$lib/stores/theme.svelte";

  const THRESHOLDS = [
    { value: 0, label: "Off" },
    ...Array.from({ length: 9 }, (_, index) => ({
      value: (index + 1) * 10,
      label: `${(index + 1) * 10}%`
    }))
  ];

  const THEMES: { value: ThemePreference; label: string }[] = [
    { value: "system", label: "System" },
    { value: "light", label: "Light" },
    { value: "dark", label: "Dark" }
  ];

  const toggles = $derived([
    {
      key: "autoStart" as const,
      title: "Launch at sign-in",
      description: "Start winpods automatically when you sign in to Windows.",
      checked: settings.current.autoStart
    },
    {
      key: "autoUpdate" as const,
      title: "Automatic updates",
      description: "Download and install new versions in the background.",
      checked: settings.current.autoUpdate
    },
    {
      key: "earDetection" as const,
      title: "Automatic ear detection",
      description: "Pause audio when you take a bud out.",
      checked: settings.current.earDetection
    }
  ]);
</script>

<div class="mx-auto flex max-w-2xl flex-col gap-5">
  <section class="flex flex-col gap-2">
    <h2 class="text-muted-foreground px-1 text-xs font-medium uppercase tracking-widest">
      General
    </h2>
    <Card flush class="divide-hairline divide-y">
      {#each toggles as toggle (toggle.key)}
        <div class="flex items-center justify-between gap-6 p-4">
          <div class="min-w-0">
            <p class="text-[13px] font-medium">{toggle.title}</p>
            <p class="text-muted-foreground mt-0.5 text-xs">{toggle.description}</p>
          </div>
          <Switch
            label={toggle.title}
            checked={toggle.checked}
            onchange={value => void settings.update({ [toggle.key]: value })}
          />
        </div>
      {/each}
    </Card>
  </section>

  <section class="flex flex-col gap-2">
    <h2 class="text-muted-foreground px-1 text-xs font-medium uppercase tracking-widest">
      Notifications
    </h2>
    <Card flush>
      <div class="flex items-center justify-between gap-6 p-4">
        <div class="min-w-0">
          <p class="text-[13px] font-medium">Low battery</p>
          <p class="text-muted-foreground mt-0.5 text-xs">
            Notify once when the battery reaches this level.
          </p>
        </div>
        <Select
          label="Low battery threshold"
          value={settings.current.lowBatteryThreshold}
          options={THRESHOLDS}
          onchange={value => void settings.update({ lowBatteryThreshold: value })}
        />
      </div>
    </Card>
  </section>

  <section class="flex flex-col gap-2">
    <h2 class="text-muted-foreground px-1 text-xs font-medium uppercase tracking-widest">
      Appearance
    </h2>
    <Card flush>
      <div class="flex items-center justify-between gap-6 p-4">
        <div class="min-w-0">
          <p class="text-[13px] font-medium">Theme</p>
          <p class="text-muted-foreground mt-0.5 text-xs">Follows Windows by default.</p>
        </div>
        <Select
          label="Theme"
          value={theme.preference}
          options={THEMES}
          onchange={value => theme.set(value)}
        />
      </div>
    </Card>
  </section>
</div>
