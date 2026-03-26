# OpenCode Desktop

Native OpenCode desktop app, built with Tauri v2.

## Prerequisites

Building the desktop app requires additional Tauri dependencies (Rust toolchain, platform-specific libraries). See the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for setup instructions.

## Development

From the repo root:

```bash
bun install
bun run --cwd packages/desktop tauri dev
```

## Build

```bash
bun run --cwd packages/desktop tauri build
```

## Troubleshooting

### OpenCode Dev storage isolation

`OpenCode Dev` uses isolated desktop storage and deep-link ownership so it can run alongside the production app without sharing auth, config, sqlite, or cache state.

- production `OpenCode` keeps using its normal `opencode` storage
- `OpenCode Dev` uses its own desktop-scoped storage and `opencode-dev://` deep links
- first launch of `OpenCode Dev` starts with empty auth/config state by design

If you need to reset only the dev app, delete its isolated desktop data/config/cache/state directories. That resets `OpenCode Dev` without touching production state.

### Rust compiler not found

If you see errors about Rust not being found, install it via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
