# CLAUDE.md

Guidance for AI agents working in this repository. Read this before making changes.

## What winpods is

A Windows desktop app that brings AirPods integration to Windows: battery levels for both buds
and the case, connection status, ear detection, low battery notifications, a movable tray widget
and a dashboard. Built with Tauri 2 (Rust backend) and SvelteKit 2 / Svelte 5 (frontend).

**winpods only runs on Windows.** The `windows` crate does not build for other targets. You can
still do most work from macOS or Linux — see [Working without Windows](#working-without-windows).

## Repository layout

```
winpods/
├── Cargo.toml                  # workspace root: shared deps, lints, profiles
├── rust-toolchain.toml         # pins the toolchain to latest stable + the Windows target
├── crates/
│   ├── apple-cp/               # winpods-apple-cp  — Apple Continuity decoding (PLATFORM INDEPENDENT)
│   ├── core/                   # winpods-core      — domain models + settings   (PLATFORM INDEPENDENT)
│   ├── bluetooth/              # winpods-bluetooth — WinRT bluetooth            (Windows only)
│   ├── media/                  # winpods-media     — WinRT media control        (Windows only)
│   └── desktop/
│       ├── src/                # SvelteKit frontend
│       ├── src-tauri/          # winpods — the Tauri app
│       └── package.json
└── docs/                       # deeper documentation, see below
```

The two platform-independent crates exist on purpose: they hold the logic that has actually gone
wrong in the past (battery aggregation, advertisement filtering, settings defaults), so it can be
unit tested on any machine rather than only in CI.

## Commands

Run frontend commands from `crates/desktop`, Rust commands from the repository root.

| Task | Command |
| --- | --- |
| Run the app (needs Windows) | `bun run tauri dev` |
| Run only the UI (any OS) | `bun run dev` → http://localhost:3000 |
| Typecheck the frontend | `bun run check` |
| Format the frontend | `bun run format` |
| Build the frontend | `bun run build` |
| Test the portable crates (any OS) | `cargo test -p winpods-apple-cp -p winpods-core` |
| Test everything (Windows) | `cargo test --workspace --all-targets` |
| Lint (Windows) | `cargo clippy --workspace --all-targets -- -D warnings` |
| Format Rust | `cargo fmt --all` |

## Working without Windows

Two things make this possible; use them rather than assuming you cannot verify your work.

**1. The UI runs standalone.** `bun run dev` serves the real UI against a mock backend
(`src/lib/ipc/mock.ts`), selected automatically when `window.__TAURI_INTERNALS__` is absent. It
keeps real state, emits the same events as Rust, and drains the batteries so live updates are
visible. A dev-only panel (bottom right) toggles the states that are awkward to reach on real
hardware: bluetooth off, disconnected, critical battery, a bud that stopped reporting.

When you change an IPC command or event, update **both** `src/lib/ipc/tauri.ts` and
`src/lib/ipc/mock.ts` — the `Backend` interface in `src/lib/ipc/backend.ts` will not let you
forget, so let the typechecker guide you.

**2. Rust cross-compiles.** The portable crates check and test natively. For the Windows-only
crates:

```sh
cargo check --target x86_64-pc-windows-msvc -p winpods-bluetooth -p winpods-media
```

That works with no extra setup. The Tauri crate additionally needs an MSVC toolchain because
`ring` compiles C:

```sh
cargo install cargo-xwin
brew install llvm                      # provides llvm-lib, which ring's build needs
PATH="/opt/homebrew/opt/llvm/bin:$PATH" \
  cargo xwin clippy --target x86_64-pc-windows-msvc --workspace --all-targets
```

Cross-checking catches real bugs — see the `Send` note below, which is invisible until it compiles.

## Architecture

The backend owns all state and pushes it to the UI. One broadcast channel of `AppEvent` carries
everything that changes; features subscribe to it, and a single bridge task forwards each event to
the webviews.

```
  AdapterWatcher ──────┐                       ┌── features::ear_detection
                       ├── AppEvent broadcast ─┼── features::low_battery
  AdvertisementWatcher ┤                       ├── features::autostart
        │              │                       ├── tray::tooltip
        └─ DeviceService                       └── features::bridge ── emit ─→ webviews
```

The UI reads state with commands and requests changes with commands, so every write has a result
it can act on. It never emits events *to* the backend.

Full detail in [docs/architecture.md](docs/architecture.md).

## Rules and gotchas

These are the things that will bite you. Most were learned by breaking them.

### WinRT needs a COM apartment

Every WinRT call must run inside a COM apartment, and tokio worker threads have none. Without it
the first call from a task fails with `CO_E_NOTINITIALIZED`. `winpods_bluetooth::com::ensure_mta()`
registers a process-wide implicit MTA and is called at the top of `run()` in
`crates/desktop/src-tauri/src/lib.rs`. Do not remove it, and call it in any new entry point that
reaches WinRT before the app has started.

### Never hold a WinRT collection across an `await`

`IIterator<T>` and `IIterable<T>` wrap a bare `IUnknown(NonNull<c_void>)` and are **not `Send`**,
even though the WinRT objects they yield are. Tauri requires command and task futures to be
`Send`, so one of these alive across an `await` makes the whole future non-`Send` and the error
message points at the command, not the real line.

Two established patterns, both in the tree:

```rust
// Collect first, then await (crates/media/src/lib.rs)
let playing: Vec<_> = { let s = manager.GetSessions()?; s.into_iter().filter(is_playing).collect() };
for session in playing { session.TryPauseAsync()?.await?; }

// Scope the iterable so it drops before the await (crates/bluetooth/src/device/discovery.rs)
let operation = {
    let requested = IIterable::<HSTRING>::from(vec![/* ... */]);
    DeviceInformation::FindAllAsyncAqsFilterAndAdditionalProperties(&filter, &requested)?
};
let found = operation.await?;
```

### Rust owns the settings

There is no store plugin and no settings write path from the UI other than the `update_settings`
command. `Settings` is one serde struct with `#[serde(default)]`, validated on load and on every
change, persisted by writing a sibling temp file and renaming it. Adding a setting means: add the
field (with a default), extend `SettingsPatch`, extend the UI type in `src/lib/ipc/types.ts`, and
add a test.

### Both windows are separate prerendered routes

`/` is the dashboard, `/widget/` is the tray widget, wired up by `url` in `tauri.conf.json`. They
are prerendered by `adapter-static` with no SPA fallback, so a route that stops being prerenderable
fails the build instead of opening a blank window. CI asserts both `build/index.html` and
`build/widget/index.html` exist.

### The design system is shared with winpods.app

Colours, radii, type stack and the frosted-glass treatment are taken from the website so the app
and the site read as one product. The tokens live in `crates/desktop/src/app.css`; use the semantic
Tailwind classes (`bg-card`, `text-muted-foreground`, `border-hairline`) and never hardcode a hex
value. See [docs/ui.md](docs/ui.md).

### Batteries are `Option`, never 0-as-missing

A bud in a closed case reports **no level**, which is different from reporting 0%. Absent readings
are `Option<Battery>` / `Battery | null` end to end. The previous version used 0 as a sentinel, so a
genuinely empty bud was indistinguishable from a missing one.

### Over-ear models are one unit, not a pair

AirPods Max have a single battery and no case, but the payload still populates the case nibble and
both battery nibbles. `AppleDeviceModel::has_case` and `AppleDeviceModel::is_single_unit` gate this;
the dashboard shows one "Battery" row, the tray tooltip one line, and the case is dropped. The
frontend mirror is `singleUnit` on `ModelArtwork` in `src/lib/models.ts`. When adding a model, set
both correctly — showing "Left 60% / Right 60%" for a single-unit device invents a distinction the
hardware does not have.

### Do not reintroduce the type-erased event dispatcher

v0.1 had a hand-rolled `EventDispatcher` with `Any`-based downcasting and synchronous callbacks, and
the backend talked to itself through Tauri's global events, re-parsing its own JSON payloads. Both
are gone. Use `tokio::sync::broadcast` with a typed enum.

## Conventions

- **Rust**: `cargo fmt` defaults. `anyhow` everywhere with `.context()` on fallible boundaries;
  no custom error enums. `unsafe_code = "deny"` at the workspace level — the one exception is the
  documented COM call. `clippy::unwrap_used` is warned on; use `expect` with a reason, or handle it.
  Tests may `unwrap` (allow it at the test module level).
- **Frontend**: prettier with `printWidth: 90`, `arrowParens: "avoid"`, `trailingComma: "none"` —
  matching v0.1's implicit style, now enforced. Svelte 5 runes only; no stores from `svelte/store`.
  Shared state lives in `src/lib/stores/*.svelte.ts` as a class instance with an idempotent
  `start()`.
- **Setup follows one shape.** Every subsystem the setup hook wires up exposes `init(...)` and
  owns its own spawning, so `lib.rs` reads as a flat list of `init` calls with no `async_runtime`
  bookkeeping at the call site:

  ```rust
  features::init(app.handle(), &state);
  tray::init(app, &state)?;
  bluetooth::init(&state);
  ```

  `init` must return promptly. `bluetooth::init` spawns and returns because attaching to the radio
  is a WinRT round trip, and blocking the setup hook leaves the windows unresponsive while it waits.
- **Comments** explain *why*, not what. Several comments in the tree record a bug that a change
  would reintroduce — do not delete those when refactoring.
- **Tests** go next to the code in a `#[cfg(test)] mod tests`. Prefer testing the portable crates,
  since those tests run everywhere.

## Further reading

- [docs/architecture.md](docs/architecture.md) — crates, event flow, threading, state ownership
- [docs/protocol.md](docs/protocol.md) — the Apple Continuity payload layout, byte by byte
- [docs/development.md](docs/development.md) — setup, cross-compiling, release process
- [docs/ui.md](docs/ui.md) — design tokens, components, adding a screen
