<script lang="ts">
  import { onMount } from "svelte";
  import Settings from "@lucide/svelte/icons/settings";
  import LayoutDashboard from "@lucide/svelte/icons/layout-dashboard";

  import DashboardPage from "$lib/views/dashboard-page.svelte";
  import SettingsPage from "$lib/views/settings-page.svelte";
  import DevPanel from "$lib/views/dev-panel.svelte";
  import { StatusMessage } from "$lib/components";
  import { Button } from "$lib/components/ui";
  import { backend } from "$lib/ipc";
  import { bluetooth, updater } from "$lib/stores";
  import { cn } from "$lib/utils";

  type Tab = "dashboard" | "settings";

  const TABS: { id: Tab; label: string; icon: typeof Settings }[] = [
    { id: "dashboard", label: "Dashboard", icon: LayoutDashboard },
    { id: "settings", label: "Settings", icon: Settings }
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
  <!-- Translucent title bar, mirroring the website's frosted nav. -->
  <header
    class="bg-nav apple-glass border-hairline flex h-12 shrink-0 items-center gap-1 border-b px-3"
  >
    {#each TABS as item (item.id)}
      <button
        onclick={() => (tab = item.id)}
        aria-current={tab === item.id ? "page" : undefined}
        class={cn(
          "flex h-8 items-center gap-2 rounded-lg px-3 text-[13px] font-medium transition-colors",
          tab === item.id
            ? "bg-card text-foreground shadow-sm"
            : "text-muted-foreground hover:text-foreground"
        )}
      >
        <item.icon class="size-4" aria-hidden="true" />
        {item.label}
      </button>
    {/each}
  </header>

  <main class="flex-1 overflow-y-auto p-6">
    {#if !bluetooth.isOn}
      <StatusMessage
        icon="bluetooth-off"
        title="Bluetooth is off"
        message="Turn bluetooth on in Windows settings to see your device."
        class="min-h-96"
      />
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
    <span class="tabular-nums">v{updater.currentVersion}</span>

    {#if updater.updateAvailable}
      <Button
        size="sm"
        loading={updater.installing}
        onclick={() => void updater.install()}
      >
        {updater.installing ? "Updating" : `Update to ${updater.latestVersion}`}
      </Button>
    {/if}
  </footer>

  {#if !backend.isLive}
    <DevPanel />
  {/if}
</div>
