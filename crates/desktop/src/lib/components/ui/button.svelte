<script lang="ts" module>
  import { tv, type VariantProps } from "tailwind-variants";

  export const buttonVariants = tv({
    base: [
      "inline-flex shrink-0 items-center justify-center gap-2 rounded-full font-medium",
      "whitespace-nowrap transition-colors select-none",
      "disabled:pointer-events-none disabled:opacity-40",
      "[&_svg]:pointer-events-none [&_svg]:shrink-0"
    ],
    variants: {
      variant: {
        // Apple's pill-shaped accent button.
        primary:
          "bg-primary text-primary-foreground hover:bg-primary/90 active:bg-primary/80",
        secondary:
          "bg-secondary text-secondary-foreground hover:bg-secondary/70 border-border border",
        ghost: "hover:bg-accent text-foreground",
        destructive:
          "bg-destructive/10 text-destructive hover:bg-destructive/15 active:bg-destructive/20",
        link: "text-link underline-offset-4 hover:underline"
      },
      size: {
        sm: "h-7 px-3 text-xs [&_svg]:size-3.5",
        md: "h-9 px-4 text-sm [&_svg]:size-4",
        lg: "h-11 px-6 text-base [&_svg]:size-5",
        icon: "size-9 [&_svg]:size-4"
      }
    },
    defaultVariants: { variant: "primary", size: "md" }
  });

  export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
  export type ButtonSize = VariantProps<typeof buttonVariants>["size"];
</script>

<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { cn } from "$lib/utils";
  import Spinner from "./spinner.svelte";

  interface Props extends HTMLButtonAttributes {
    variant?: ButtonVariant;
    size?: ButtonSize;
    loading?: boolean;
  }

  let {
    variant = "primary",
    size = "md",
    loading = false,
    disabled,
    class: className,
    children,
    ...rest
  }: Props = $props();
</script>

<button
  class={cn(buttonVariants({ variant, size }), className)}
  disabled={disabled || loading}
  {...rest}
>
  {#if loading}
    <Spinner />
  {/if}
  {@render children?.()}
</button>
