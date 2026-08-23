<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  import DashboardPage from "$lib/views/dashboard-page.svelte";
  import SettingsPage from "$lib/views/settings-page.svelte";
  import DevPanel from "$lib/views/dev-panel.svelte";
  import { EmptyState } from "$lib/components";
  import { Button } from "$lib/components/ui";
  import { backend } from "$lib/ipc";
  import { bluetooth, updater } from "$lib/stores";
  import { cn } from "$lib/utils";

  type Tab = "dashboard" | "settings";

  const TABS: { id: Tab; label: string }[] = [
    { id: "dashboard", label: "Device" },
    { id: "settings", label: "Settings" }
  ];

  let tab = $state<Tab>("dashboard");

  onMount(() => {
    void updater.start();
  });
</script>

<svelte:head>
  <title>winpods</title>
</svelte:head>

<div class="bg-surface flex h-screen flex-col overflow-hidden">
  <!--
    Frosted title bar with a macOS-style segmented control, centred so it reads as a window
    chrome affordance rather than web navigation.
  -->
  <header
    class="bg-nav apple-glass border-hairline relative flex h-12 shrink-0 items-center
           justify-center border-b"
  >
    <div class="bg-secondary inline-flex gap-0.5 rounded-[9px] p-[3px]">
      {#each TABS as item (item.id)}
        <button
          onclick={() => (tab = item.id)}
          aria-current={tab === item.id ? "page" : undefined}
          class={cn(
            "rounded-md px-4 py-1 text-[13px] font-medium transition-all duration-200",
            tab === item.id
              ? "bg-card text-foreground shadow-sm"
              : "text-muted-foreground hover:text-foreground"
          )}
        >
          {item.label}
        </button>
      {/each}
    </div>
  </header>

  <main class="flex-1 overflow-y-auto px-8 py-6">
    {#if !bluetooth.isOn}
      <div
        class="flex min-h-full items-center justify-center"
        in:fade={{ duration: 200 }}
      >
        <EmptyState
          icon="bluetooth-off"
          title="Bluetooth is off"
          description="winpods needs bluetooth to read battery levels from your device."
          steps={[
            "Open Windows Settings, then Bluetooth & devices.",
            "Turn Bluetooth on.",
            "winpods picks your device up automatically."
          ]}
        />
      </div>
    {:else if tab === "dashboard"}
      <DashboardPage />
    {:else}
      <SettingsPage />
    {/if}
  </main>

  <footer
    class="border-hairline text-muted-foreground flex h-10 shrink-0 items-center
           justify-between border-t px-6 text-xs"
  >
    <span class="tabular-nums">Version {updater.currentVersion}</span>

    {#if updater.updateAvailable}
      <Button
        size="sm"
        loading={updater.installing}
        onclick={() => void updater.install()}
      >
        {updater.installing ? "Updating…" : `Update to ${updater.latestVersion}`}
      </Button>
    {/if}
  </footer>

  {#if !backend.isLive}
    <DevPanel />
  {/if}
</div>
