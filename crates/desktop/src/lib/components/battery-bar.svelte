<script lang="ts">
  import { TONE_FILL, batteryTone } from "$lib/battery";
  import { cn } from "$lib/utils";

  interface Props {
    level: number | null;
    charging?: boolean;
    /** Renders the track for the widget's dark panel. */
    onDark?: boolean;
    class?: string;
  }

  let { level, charging = false, onDark = false, class: className }: Props = $props();

  const tone = $derived(batteryTone(level, charging));
  // A 0% battery still needs a sliver of fill, otherwise an empty bar reads as "no reading".
  const width = $derived(level === null ? 0 : Math.max(level, 2));
</script>

<div
  class={cn(
    "h-1.5 w-full overflow-hidden rounded-full",
    onDark ? "bg-white/15" : "bg-foreground/10",
    className
  )}
  role="progressbar"
  aria-valuenow={level ?? undefined}
  aria-valuemin={0}
  aria-valuemax={100}
>
  <div
    class={cn(
      "h-full rounded-full transition-[width] duration-700 ease-out",
      TONE_FILL[tone]
    )}
    style="width: {width}%"
  ></div>
</div>
