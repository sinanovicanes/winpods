<script lang="ts">
  import Zap from "@lucide/svelte/icons/zap";
  import { cn } from "$lib/utils";

  interface Props {
    /** Charge level as a percentage, or null when the device reported none. */
    level: number | null;
    charging?: boolean;
    /** Renders the pill on a dark translucent panel, as the widget does. */
    onDark?: boolean;
    class?: string;
  }

  let { level, charging = false, onDark = false, class: className }: Props = $props();

  // Charging always reads as healthy: the level is on its way up, so colouring it red is noise.
  const tone = $derived.by(() => {
    if (level === null) return "unknown";
    if (charging) return "good";
    if (level <= 10) return "critical";
    if (level <= 20) return "low";
    return "good";
  });

  const fillClass = $derived(
    {
      good: "bg-battery-good",
      low: "bg-battery-low",
      critical: "bg-battery-critical",
      unknown: "bg-transparent"
    }[tone]
  );

  const textClass = $derived.by(() => {
    if (onDark) return "text-white/90";
    if (tone === "critical") return "text-battery-critical";
    if (tone === "low") return "text-battery-low";
    return "text-foreground";
  });
</script>

{#if level === null}
  <!-- No reading. A bud in a closed case reports nothing, which is not the same as 0%. -->
  <span
    class={cn(
      "text-xs tabular-nums",
      onDark ? "text-white/40" : "text-muted-foreground",
      className
    )}
  >
    &mdash;
  </span>
{:else}
  <span class={cn("inline-flex items-center gap-1.5", className)}>
    <!-- A battery glyph drawn from two rounded rects plus a nub, matching Apple's proportions. -->
    <span
      class={cn(
        "relative flex h-[13px] w-[26px] items-center rounded-[4px] border p-[2px]",
        onDark ? "border-white/40" : "border-foreground/30"
      )}
    >
      <span
        class={cn("h-full rounded-[2px] transition-all duration-500", fillClass)}
        style="width: {Math.max(level, 2)}%"
      ></span>
      <span
        class={cn(
          "absolute -right-[3px] h-[5px] w-[2px] rounded-r-[1px]",
          onDark ? "bg-white/40" : "bg-foreground/30"
        )}
      ></span>
    </span>

    <span class={cn("text-xs font-medium tabular-nums", textClass)}>{level}%</span>

    {#if charging}
      <Zap
        class={cn("size-3 fill-current", onDark ? "text-white/90" : "text-battery-good")}
        aria-label="Charging"
      />
    {/if}
  </span>
{/if}
