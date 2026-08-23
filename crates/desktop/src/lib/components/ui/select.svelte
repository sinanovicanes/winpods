<script lang="ts" generics="T extends string | number">
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import { cn } from "$lib/utils";

  interface Option {
    value: T;
    label: string;
  }

  interface Props {
    value: T | null;
    options: Option[];
    placeholder?: string;
    disabled?: boolean;
    label: string;
    onchange?: (value: T) => void;
    class?: string;
  }

  let {
    value,
    options,
    placeholder,
    disabled = false,
    label,
    onchange,
    class: className
  }: Props = $props();

  /**
   * The native `<select>` is intentional: it gets the OS's own dropdown, which is what makes
   * this feel native on Windows, and it is keyboard accessible for free.
   */
  function handleChange(event: Event) {
    const raw = (event.currentTarget as HTMLSelectElement).value;
    const match = options.find(option => String(option.value) === raw);

    if (match) {
      onchange?.(match.value);
    }
  }
</script>

<div class={cn("relative inline-flex items-center", className)}>
  <select
    aria-label={label}
    {disabled}
    value={value === null ? "" : String(value)}
    onchange={handleChange}
    class={cn(
      "bg-secondary text-secondary-foreground border-border h-9 appearance-none",
      "rounded-lg border py-0 pr-8 pl-3 text-sm font-medium",
      "transition-colors hover:bg-accent disabled:opacity-40"
    )}
  >
    {#if placeholder}
      <option value="" disabled>{placeholder}</option>
    {/if}
    {#each options as option (option.value)}
      <option value={String(option.value)}>{option.label}</option>
    {/each}
  </select>
  <ChevronDown
    class="text-muted-foreground pointer-events-none absolute right-2.5 size-4"
    aria-hidden="true"
  />
</div>
