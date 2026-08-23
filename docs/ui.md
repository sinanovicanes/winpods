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
- `cursor: pointer` on interactive elements is set once in `app.css`, not repeated as a utility.

**Watch out: `secondary`, `muted`, `accent` and `surface` are all `#f5f5f7` in light mode.** A
`bg-secondary` element sitting on the page background is therefore invisible. For something that
has to read against the surface, use `bg-card` with `border-hairline`, or a translucent tint like
`bg-foreground/[0.07]`.

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

`src/lib/components/` holds the domain components:

| Component | Used by | Notes |
| --- | --- | --- |
| `battery-card.svelte` | dashboard | Label, large percentage, bar, in-ear/charging glyphs |
| `battery-stat.svelte` | widget | The compact form: percentage and a thin bar, no caption |
| `battery-bar.svelte` | both | Shared track; `onDark` for the widget panel |
| `device-artwork.svelte` | both | `variant="hero"` for the dashboard, `"bud"` for the widget |
| `empty-state.svelte` | both | Icon disc, title, description, optional numbered `steps` and actions |

Battery colouring is centralised in `src/lib/battery.ts` (`batteryTone`, `TONE_FILL`, `TONE_TEXT`,
`TONE_TEXT_ON_DARK`) so a threshold is defined once. A charging battery always reads as healthy,
and `null` — no reading at all — is a distinct tone from 0%.

Use `cn()` from `$lib/utils` to merge classes; it lets a caller's utility override a component
default.

## Adding a screen

1. Add a component under `src/lib/views/`.
2. Reach state through the stores in `src/lib/stores`, never `invoke` directly from a component.
3. If you need new backend data, add the command to **all three** of `src/lib/ipc/backend.ts`
   (the interface), `tauri.ts` and `mock.ts`. The interface makes forgetting the mock a type error.
4. `bun run check`, then look at it with `bun run dev`.

## Motion

Transitions are decorative and deliberately small: battery bars ease their width over 700ms, cards
fade or fly in by 8px when a device connects, and the charging bolt pulses. `app.css` honours
`prefers-reduced-motion: reduce` globally, so nothing needs to opt out individually.

## Sizing

The dashboard window is 860×640 and non-resizable; the widget is 300×125, frameless and
transparent. Both are fixed in `tauri.conf.json`, so layouts can assume those dimensions.

That fixed height is a real constraint for the **device screen**: after the title bar and footer
there are about **504px** of usable content height, and that screen centres its content vertically.
Centred content that overflows gets **clipped at both ends rather than scrolling**, so a screen that
grows past 504px silently loses its bottom row. The settings screen is not centred, so it scrolls
normally and may grow past the window -- it already does by about 60px. Check it after any layout
change:

```js
// in the devtools console
const m = document.querySelector("main");
m.scrollHeight > m.clientHeight; // must be false
```

Note when screenshotting the widget with headless Chrome: Chrome enforces a minimum window size, so
`--window-size=300,125` silently renders larger and crops. Override the viewport through the
DevTools Protocol (`Emulation.setDeviceMetricsOverride`) to get a true 300×125 render.
