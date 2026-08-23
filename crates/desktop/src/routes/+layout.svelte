<script lang="ts">
  import { onMount } from "svelte";

  import "../app.css";
  import { bluetooth, devices, settings, theme } from "$lib/stores";

  let { children } = $props();

  // Both windows need the same backend state, so the shared stores start here rather than in
  // each route. Every `start` is idempotent.
  onMount(() => {
    theme.start();
    void bluetooth.start();
    void devices.start();
    void settings.start();
  });
</script>

{@render children()}
