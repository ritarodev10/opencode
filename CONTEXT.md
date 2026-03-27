# OpenCode Dev Context

This fork exists for personal desktop use with two goals:

1. Run a separate desktop build named `OpenCode Dev`
2. Use multiple GitHub Copilot accounts in the same fork

## What changed in this fork

### Multi-account GitHub Copilot support

This fork adds alias-based auth support so more than one GitHub Copilot account can be configured at once.

- added provider alias auth support through `auth_provider`
- allows one Copilot-backed provider to reuse auth flow from another provider driver
- enables separate Copilot entries for different accounts/models
- updates provider UI so extra Copilot accounts can be added from settings

Intended use:
- account A for one model family such as Haiku
- account B for another model family such as Opus

### OpenCode Dev desktop isolation

This fork changes the desktop app so the dev build can run side-by-side with the production OpenCode app.

- app name: `OpenCode Dev`
- bundle id: `ai.opencode.desktop.dev`
- deep-link scheme: `opencode-dev://`
- isolated data/config/cache/state roots for the desktop sidecar
- separate sqlite/auth/config storage from production OpenCode

This prevents the dev desktop app from sharing or corrupting production app state.

## Important behavior

- The desktop bundle name is `OpenCode Dev.app`
- The executable inside the app bundle is still named `OpenCode`
- On macOS, use `open -n` if production OpenCode is already running

## Main files changed for the dev desktop fork

- `packages/desktop/src-tauri/src/app_paths.rs`
- `packages/desktop/src-tauri/src/cli.rs`
- `packages/desktop/src-tauri/src/lib.rs`
- `packages/desktop/src-tauri/tauri.conf.json`
- `packages/app/src/pages/layout/deep-links.ts`
- `packages/app/src/pages/layout/helpers.test.ts`
- `packages/app/src/components/dialog-connect-provider.tsx`

## Run commands

From repo root:

```bash
bun run dev:desktop
```

Built debug app:

```bash
open -n "packages/desktop/src-tauri/target/debug/bundle/macos/OpenCode Dev.app"
```

## Maintenance

For how to keep this fork close to upstream while preserving the custom multi-account and desktop-isolation features, see `FORK_MAINTENANCE.md`.
