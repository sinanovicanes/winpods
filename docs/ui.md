# UI

## Design system

The tokens come from [winpods.app](https://winpods.app) so the app and the website read as one
product. They are defined in `crates/desktop/src/app.css` as CSS custom properties on `:root` and
`.dark`, then mapped into Tailwind v4 via `@theme inline`.

The palette is Apple's, not Tailwind's:

| Token | Light | Dark | Use |
| --- | --- | --- | --- |
| `background` | `#fff` | `#000` | Window background |
| `foreground` | `#1d1d1f` | `#f5f5f7` | Body text |
| `card` | `#fff` | `#1d1d1f` | Panels |
| `surface` | `#f5f5f7` | `#101012` | The area behind cards |
| `primary` | `#0071e3` | `#0071e3` | Accent, primary buttons |
| `muted-foreground` | `#6e6e73` | `#86868b` | Secondary text |
| `border` | `#d2d2d7` | `#424245` | Controls |
| `hairline` | `#0000001f` | `#ffffff1f` | Card edges, dividers |
| `destructive` | `#e30000` | `#ff453a` | Destructive actions |

Plus `battery-good` / `battery-low` / `battery-critical` for the battery states.

**Always use the semantic class, never a hex value or a Tailwind palette colour.** `bg-card`,
`text-muted-foreground`, `border-hairline`. That is what keeps dark mode working without a second
set of classes.

Other conventions worth knowing:

- `--radius` is `0.75rem`. `rounded-lg` is the default for controls, `rounded-xl` for cards,
  `rounded-full` for buttons (Apple's pill shape).
- The font stack starts with SF Pro and falls through to **Segoe UI Variable Display**, which is
  what actually renders on Windows.
- `letter-spacing: -0.02em` on `html`. Apple sets type slightly tight; without it the UI reads
  noticeably wider.
- `apple-glass` is the frosted treatment (`backdrop-filter: saturate(180%) blur(20px)`), used by
  the dashboard title bar and the widget panel.

## Theme

Three states: `light`, `dark`, `system` (the default). `app.html` applies the resolved theme to
`<html>` **before first paint** so the window never flashes the wrong colour scheme; `theme.svelte.ts`
keeps it in step afterwards and follows the OS while the preference is `system`.

## Components

`src/lib/components/ui/` holds shadcn-style primitives — same token system and API shape, written
by hand rather than pulled from the registry:

| Component | Notes |
| --- | --- |
| `button.svelte` | `tailwind-variants`; variants `primary`/`secondary`/`ghost`/`destructive`/`link` |
| `card.svelte` | `flush` removes padding so the card can hold its own sections |
| `switch.svelte` | `role="switch"` on a button; requires a `label` for accessibility |
| `select.svelte` | Wraps the **native** `<select>` on purpose — it gets the OS dropdown |
| `spinner.svelte` | Used by `Button`'s `loading` state |

`src/lib/components/` holds the domain components: `battery-indicator`, `device-artwork`,
`status-message`. All three take an `onDark` or equivalent so they work on the widget's dark panel.

Use `cn()` from `$lib/utils` to merge classes; it lets a caller's utility override a component
default.

## Adding a screen

1. Add a component under `src/lib/views/`.
2. Reach state through the stores in `src/lib/stores`, never `invoke` directly from a component.
3. If you need new backend data, add the command to **all three** of `src/lib/ipc/backend.ts`
   (the interface), `tauri.ts` and `mock.ts`. The interface makes forgetting the mock a type error.
4. `bun run check`, then look at it with `bun run dev`.

## Sizing

The dashboard window is 860×640 and non-resizable; the widget is 300×125, frameless and
transparent. Both are fixed in `tauri.conf.json`, so layouts can assume those dimensions — but keep
the dashboard's content in a `max-w-2xl` column so it stays centred and readable.

Note when screenshotting the widget with headless Chrome: Chrome enforces a minimum window size, so
`--window-size=300,125` silently renders larger and crops. Override the viewport through the
DevTools Protocol (`Emulation.setDeviceMetricsOverride`) to get a true 300×125 render.
