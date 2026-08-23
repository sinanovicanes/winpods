# Development

## Prerequisites

- **Rust** — the toolchain is pinned by `rust-toolchain.toml`, so `rustup` installs the right
  channel, components and target automatically.
- **Bun** — used for the frontend. `npm`/`pnpm` work too, but `bun.lock` is what CI installs from.
- **Windows 10 or 11** to *run* the app. Not needed to work on it; see below.

## Running

```sh
cd crates/desktop
bun install
bun run tauri dev      # the whole app — Windows only
bun run dev            # the UI alone, any OS
```

`bun run dev` serves the real UI against the mock backend and prints to http://localhost:1420. The
dashboard is `/`, the widget is `/widget/`.

## Working without Windows

### The UI

`bun run dev` needs nothing but a browser. The backend is faked by `src/lib/ipc/mock.ts`, chosen
automatically when `window.__TAURI_INTERNALS__` is absent. It holds real state, emits the same
events the Rust backend does, and slowly drains the batteries so live updates are visible.

The **Mock** button in the bottom-right corner opens a panel for the states that are hard to
reproduce on real hardware: bluetooth off, a disconnected device, specific battery levels, charging,
in-ear, and a bud that stopped reporting. It only renders when the mock backend is active, so it can
never appear in a shipped build.

The widget is a transparent 300×125 window. In a browser it renders on whatever is behind it; that
is expected.

### The Rust code

The portable crates build and test natively:

```sh
cargo test -p winpods-apple-cp -p winpods-core
```

The Windows-only library crates cross-check with no extra setup, because they are pure Rust
bindings:

```sh
rustup target add x86_64-pc-windows-msvc
cargo check --target x86_64-pc-windows-msvc -p winpods-bluetooth -p winpods-media
```

The Tauri crate needs more, because `ring` (via the updater plugin) compiles C and therefore wants
an MSVC toolchain:

```sh
cargo install cargo-xwin
brew install llvm    # ring's build invokes llvm-lib

PATH="/opt/homebrew/opt/llvm/bin:$PATH" \
  cargo xwin clippy --target x86_64-pc-windows-msvc --workspace --all-targets
```

`cargo-xwin` downloads the Windows SDK headers and import libraries on first use. This catches
things that are otherwise invisible until CI — the non-`Send` WinRT collection problem described in
[CLAUDE.md](../CLAUDE.md) was found exactly this way.

Cross-*checking* works; cross-*linking* a runnable `.exe` is not part of the workflow. Use CI or a
Windows machine for that.

## Verifying a change

| What changed | Run |
| --- | --- |
| Protocol decoding, models, settings | `cargo test -p winpods-apple-cp -p winpods-core` |
| Any Rust | `cargo xwin clippy --target x86_64-pc-windows-msvc --workspace --all-targets` |
| Any frontend code | `bun run check` then `bun run build` |
| UI appearance | `bun run dev` and look at it |
| Anything, before pushing | `cargo fmt --all` and `bun run format` |

## Type checking

The frontend runs **TypeScript 7** through `svelte-check --tsgo`. That needs two compilers
installed side by side, which is why `package.json` has both:

```json
"typescript": "6.0.3",
"@typescript/native": "npm:typescript@7.0.2"
```

`typescript` (6.x) is what Vite and the Svelte plugin use for transforms; `@typescript/native` is an
npm alias for TypeScript 7, which `--tsgo` runs for the actual checking. Dropping either one breaks
`bun run check`: `svelte-check` refuses to start on TypeScript 7 alone.

## CI

`.github/workflows/ci.yaml` has two jobs:

- **Rust** on `windows-latest`: `cargo fmt --check`, `clippy -D warnings`, `cargo test`.
- **Frontend** on `ubuntu-latest`: `format:check`, `check`, `build`, and an assertion that both
  `build/index.html` and `build/widget/index.html` were prerendered — a route that stops being
  prerenderable would otherwise ship as a blank window.

## Releasing

1. Bump the version in **three** places, which must agree:
   - `Cargo.toml` (`workspace.package.version`)
   - `crates/desktop/package.json`
   - `crates/desktop/src-tauri/tauri.conf.json`
2. Commit and tag `vX.Y.Z`.
3. Pushing the tag runs `.github/workflows/release.yaml`, which builds NSIS and MSI bundles and
   opens a **draft** release.
4. Review the draft, then publish.

Updater artifacts are signed with `TAURI_SIGNING_PRIVATE_KEY`; the matching public key is in
`tauri.conf.json` and must not change without shipping a migration path, or existing installs will
reject every future update.
