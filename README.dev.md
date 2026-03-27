# OpenCode Dev

This fork includes a personal-use desktop variant called `OpenCode Dev`.

## Features added in this fork

### 1. Separate desktop app

`OpenCode Dev` is isolated from the production OpenCode desktop app.

- different bundle id
- different deep-link scheme
- different local app storage
- safe to run alongside `/Applications/OpenCode.app`

### 2. Multiple GitHub Copilot accounts

This fork supports alias-style Copilot providers so you can use multiple Copilot-backed accounts in one setup.

Examples:
- one Copilot account for Haiku-style usage
- another Copilot account for Opus-style usage

## Run in development

From `opencode-fork`:

```bash
bun run dev:desktop
```

## Run the built app

```bash
open -n "packages/desktop/src-tauri/target/debug/bundle/macos/OpenCode Dev.app"
```

If nothing appears, quit the production OpenCode app first or launch the real binary directly:

```bash
"packages/desktop/src-tauri/target/debug/bundle/macos/OpenCode Dev.app/Contents/MacOS/OpenCode"
```

## Where the app is

Built debug app bundle:

`packages/desktop/src-tauri/target/debug/bundle/macos/OpenCode Dev.app`

## Notes

- Finder shows the app as `OpenCode Dev`
- the binary inside the bundle is still named `OpenCode`
- the dev app uses `opencode-dev://` deep links
- the dev app stores its own desktop state separately from production

## Keeping This Fork Updated

For the recommended upstream-sync and patch-maintenance workflow, see `FORK_MAINTENANCE.md`.
