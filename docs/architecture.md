# Architecture

## Crates

| Crate | Package | Platform | Purpose |
| --- | --- | --- | --- |
| `crates/apple-cp` | `winpods-apple-cp` | any | Decodes Apple Continuity proximity pairing payloads |
| `crates/core` | `winpods-core` | any | Domain models (`Battery`, `DeviceProperties`) and settings |
| `crates/bluetooth` | `winpods-bluetooth` | Windows | Radio, BLE advertisements, paired devices |
| `crates/media` | `winpods-media` | Windows | Pause/resume system media sessions |
| `crates/desktop/src-tauri` | `winpods` | Windows | The Tauri app: commands, features, tray |

Dependencies flow one way:

```
winpods-apple-cp ─┬─→ winpods-core ─┬─→ winpods-bluetooth ─┐
                  │                 │                      ├─→ winpods (app)
                  └─────────────────┘   winpods-media ─────┘
```

`apple-cp` and `core` deliberately depend on nothing platform specific. That is what lets
`cargo test -p winpods-apple-cp -p winpods-core` run on any machine, and it covers the logic that
has historically been wrong: battery aggregation, the advertisement plausibility filter, settings
defaults and clamping.

## Where state lives

| State | Owner | Read by the UI via | Changed by the UI via |
| --- | --- | --- | --- |
| Adapter on/off | `AdapterWatcher` (cached in an `AtomicBool`) | `get_adapter_state` | — (OS owns it) |
| Selected device + readings | `DeviceService` | `get_current_device` | `select_device`, `clear_device_selection` |
| Settings | `SettingsService` | `get_settings` | `update_settings` |
| Update status | `UpdateService` | `get_update_status` | `install_update` |
| Theme | the UI (`localStorage`) | — | — |

Everything except the theme is owned by Rust. The UI holds a mirror kept current by events, and
never a second source of truth.

## Event flow

One `tokio::sync::broadcast` channel of `AppEvent` is the backbone:

```
AdapterWatcher ─── AdapterState ──┐
                                  │   bluetooth::spawn_adapter_task
AdvertisementWatcher ─ Advertisement ─── DeviceService::apply_advertisement
                                  │
Device (WinRT handlers) ─ DeviceEvent ── DeviceService watcher task
                                  │
SettingsService ──────────────────┤
                                  ▼
                      broadcast::Sender<AppEvent>
                                  │
    ┌──────────────┬──────────────┼──────────────┬────────────────┐
    ▼              ▼              ▼              ▼                ▼
ear_detection  low_battery   autostart     tray::tooltip      bridge
                                                                 │
                                                          app.emit(...) → both webviews
```

Why a single typed channel rather than Tauri's global events, which v0.1 used for this:

- **Typed payloads.** Features receive the value, not a JSON string to re-parse. v0.1 called
  `serde_json::from_str(event.payload())` inside each feature and silently `return`ed on failure.
- **No echo.** Tauri global events broadcast back to the sender, which forced every setter to
  early-return on an unchanged value to avoid a loop.
- **One place that knows about the UI.** Only `features::bridge` calls `emit`.

### Lag handling

`broadcast` receivers can fall behind. Both consumers of high-volume streams treat a lag as
skippable, because every event carries the complete current value rather than a delta — the next
event supersedes anything missed. This is explicit in `features::bridge` and
`bluetooth::spawn_advertisement_task`; do not turn a lag into an error.

## Setup

`run()` in `crates/desktop/src-tauri/src/lib.rs` registers the plugins and commands, then the setup
hook wires up the three subsystems. Each exposes `init(...)` and handles its own task spawning:

| Call | Does |
| --- | --- |
| `features::init` | Starts the event-bus consumers: bridge, autostart, ear detection, low battery, updater |
| `tray::init` | Builds the tray icon and menu, and starts the tooltip task |
| `bluetooth::init` | Spawns the adapter and advertisement tasks, then picks up a connected device |

State is managed **before** any of them run. The windows declared in `tauri.conf.json` exist before
the setup hook, so their webviews can invoke commands while setup is still in progress; managing the
state first is what keeps those early calls from failing with "state not managed".

## Threading

- The Tauri/tokio runtime runs everything. There is no dedicated bluetooth thread.
- WinRT invokes event handlers on **its own threadpool threads**. Those handlers do the minimum
  possible work: an optional filter, a copy, and a `broadcast::Sender::send`. Nothing blocks and
  nothing awaits inside a WinRT callback.
- A process-wide implicit COM MTA is registered once at startup
  (`winpods_bluetooth::com::ensure_mta`). Every WinRT call needs an apartment and tokio worker
  threads do not create one.
- `Send` is a hard constraint. See the note in [CLAUDE.md](../CLAUDE.md#never-hold-a-winrt-collection-across-an-await) —
  WinRT collection types are not `Send` and must not be alive across an `await`.

## The advertisement hot path

This is the busiest code in the app: Windows delivers a callback for every BLE advertisement in
radio range, several per second per nearby device. The filtering is layered so the expensive work
happens last.

1. `AdvertisementWatcher` is constructed with Apple's company id, so payloads from other vendors
   are skipped **before** their bytes are copied.
2. The handler returns immediately if nothing is subscribed.
3. `ProximityPairingMessage::from_bytes` rejects anything that is not a proximity pairing message
   (length and header check, no allocation).
4. `DeviceService::apply_advertisement` takes a **read** lock first, because almost every
   advertisement is rejected and taking a write lock each time would serialise the path. It checks,
   in order: a device is selected; it is connected; the model matches; the readings are plausible;
   the readings actually changed.
5. Only then does it take the write lock and publish.

Step 4's plausibility check (`DeviceProperties::is_plausible_update`) exists because several
devices of the same model can be in range and they advertise identically. A reading that jumps more
than 50 dBm or 20 percentage points is treated as a different pair and dropped.

## Frontend

- SvelteKit 2 with `adapter-static`. Both routes are prerendered; `ssr = false`.
- `/` is the dashboard window, `/widget/` is the tray widget, selected by `url` in
  `tauri.conf.json`. v0.1 loaded one entry point into both windows and branched on the window label
  at runtime; separate routes replace that.
- All backend access goes through the `Backend` interface (`src/lib/ipc/backend.ts`), which has two
  implementations: `tauri.ts` and `mock.ts`. The mock is what makes the UI runnable without Windows.
- Shared state is a class instance per concern in `src/lib/stores/*.svelte.ts`, using Svelte 5
  runes, with an idempotent `start()` called from the root layout.
