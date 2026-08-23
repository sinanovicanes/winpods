<script lang="ts">
  import { cn } from "$lib/utils";

  interface Props {
    checked: boolean;
    disabled?: boolean;
    /** Accessible name, required because the switch renders no visible text of its own. */
    label: string;
    onchange?: (checked: boolean) => void;
    class?: string;
  }

  let { checked, disabled = false, label, onchange, class: className }: Props = $props();
</script>

<!--
  A plain button with role="switch" rather than a checkbox: it needs no form value, and this
  keeps the keyboard and screen reader behaviour correct without a headless dependency.
-->
<button
  type="button"
  role="switch"
  aria-checked={checked}
  aria-label={label}
  {disabled}
  onclick={() => onchange?.(!checked)}
  class={cn(
    "relative inline-flex h-[31px] w-[51px] shrink-0 cursor-pointer items-center rounded-full",
    "transition-colors duration-200 disabled:cursor-not-allowed disabled:opacity-40",
    checked ? "bg-battery-good" : "bg-border",
    className
  )}
>
  <span
    class={cn(
      "pointer-events-none block size-[27px] rounded-full bg-white shadow-sm",
      "transition-transform duration-200",
      checked ? "translate-x-[22px]" : "translate-x-[2px]"
    )}
  ></span>
</button>
