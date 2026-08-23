<script lang="ts">
  import { artworkFor } from "$lib/models";
  import { cn } from "$lib/utils";
  import type { AppleDeviceModel } from "$lib/ipc";

  interface Props {
    model: AppleDeviceModel | undefined;
    /** `bud` for the widget's small image, `hero` for the dashboard's product shot. */
    variant?: "bud" | "hero";
    /**
     * Height utility applied to the `<img>` elements themselves.
     *
     * Sizing the images directly rather than a wrapper matters: a percentage height on an image
     * inside a sized wrapper does not resolve reliably, which silently collapsed this layout.
     */
    imgClass?: string;
    class?: string;
  }

  let { model, variant = "bud", imgClass, class: className }: Props = $props();

  const artwork = $derived(artworkFor(model));
</script>

{#if variant === "hero"}
  <img
    src={artwork.hero}
    alt={artwork.name}
    class={cn("object-contain", imgClass, className)}
  />
{:else if artwork.pair}
  <!-- One bud image mirrored, so a single asset renders a left/right pair. -->
  <span class={cn("flex items-end gap-1", className)}>
    <img
      src={artwork.bud}
      alt=""
      class={cn("w-auto object-contain", imgClass)}
      aria-hidden="true"
    />
    <img
      src={artwork.bud}
      alt={artwork.name}
      class={cn("w-auto -scale-x-100 object-contain", imgClass)}
    />
  </span>
{:else}
  <img
    src={artwork.bud}
    alt={artwork.name}
    class={cn("w-auto object-contain", imgClass, className)}
  />
{/if}
