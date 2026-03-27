# Fork Maintenance Guide

This fork should stay close to upstream while preserving a small set of personal-use features.

## Goal

Keep using the latest OpenCode updates when possible, but preserve these fork-only capabilities:

1. `OpenCode Dev` desktop isolation
2. multiple GitHub Copilot accounts
3. multiple OpenAI/ChatGPT accounts
4. OpenAI OAuth model behavior needed by this fork

The safest way to do that is to keep upstream clean and replay a small patch stack on top.

## Branch Strategy

Use these branches:

- `upstream-dev`
  - clean mirror of upstream `dev`
  - no personal edits
- `dev`
  - your actual fork branch
  - based on `upstream-dev` plus your patch commits

Optional topic branches if you want cleaner review history:

- `patch/desktop-isolation`
- `patch/copilot-multi-account`
- `patch/openai-multi-account`

## Recommended Commit Layout

Keep your fork-only changes in small, isolated commits like this:

1. `desktop: isolate OpenCode Dev state and deep links`
2. `provider: support multiple GitHub Copilot accounts`
3. `provider: support multiple OpenAI accounts`
4. `openai: align oauth model behavior for fork`
5. `docs: add fork context and maintenance notes`

That layout makes rebases and cherry-picks much easier.

## Upstream Sync Workflow

### One-time setup

Check remotes:

```bash
git remote -v
```

If needed, add upstream:

```bash
git remote add upstream <UPSTREAM_GIT_URL>
```

Fetch everything:

```bash
git fetch origin --tags
git fetch upstream --tags
```

Create the clean upstream tracking branch:

```bash
git checkout -B upstream-dev upstream/dev
```

Return to your fork branch:

```bash
git checkout dev
```

### Regular update cycle

Refresh upstream:

```bash
git fetch upstream --tags
git checkout upstream-dev
git reset --hard upstream/dev
```

Rebase your fork onto the refreshed upstream:

```bash
git checkout dev
git rebase upstream-dev
```

If rebase is too messy for a particular update, use selective cherry-picking instead:

```bash
git checkout -B dev-new upstream-dev
git cherry-pick <fork-patch-1>
git cherry-pick <fork-patch-2>
git cherry-pick <fork-patch-3>
```

Then replace `dev` with the rebuilt branch once verified.

## How To Decide Keep vs Skip

For every upstream change that touches the same area:

- **Take upstream** when it improves the same area and your custom behavior still works
- **Skip upstream change** when it breaks a fork-only capability and is not important enough to adapt immediately
- **Drop your patch** when upstream now supports the feature natively
- **Rework your patch** when upstream changes internals but the capability is still needed

## Use Cherry-Pick For Safer Updates

If upstream changed a lot, do not force a full merge immediately.

Prefer:

```bash
git checkout -B dev-new upstream-dev
git cherry-pick <only the fork commits you still need>
```

This is safer than dragging old history through large upstream refactors.

## Regression Checklist After Every Sync

Run these checks before trusting the updated fork:

### Automated

From `packages/opencode`:

```bash
bun test test/cli/plugin-auth-picker.test.ts test/provider/provider.test.ts test/session/llm.test.ts test/config/config.test.ts
bun typecheck
```

From `packages/app`:

```bash
bun typecheck
```

From `packages/ui`:

```bash
bun typecheck
```

From `packages/desktop` when desktop code changed:

```bash
RUST_TARGET=aarch64-apple-darwin bun run predev
bun run tauri build --debug
```

### Manual

Verify these in `OpenCode Dev`:

1. app launches successfully
2. app uses isolated dev storage, not production storage
3. second GitHub Copilot account can connect and send requests
4. second OpenAI account can connect and send requests
5. OpenAI alias models appear as expected
6. production OpenCode still works separately

## Current Fork Features To Preserve

### Desktop isolation

- separate desktop identity: `ai.opencode.desktop.dev`
- separate deep-link scheme: `opencode-dev://`
- separate data/config/cache/state roots

### Multi-account auth

- `auth_provider` alias support
- extra GitHub Copilot account flow
- extra OpenAI account flow

### Docs

- `CONTEXT.md`
- `README.dev.md`
- this file

## Practical Rule

Do not try to preserve every old customization forever.

Instead:

- keep the fork patch stack small
- let upstream replace fork code when it becomes good enough
- only carry forward the capabilities you still actively use

## Recommended Future Routine

- sync from upstream regularly instead of after huge drift
- keep custom patches focused and topic-based
- avoid mixing fork features with unrelated cleanup in one commit
- after each successful sync, rebuild `OpenCode Dev` and smoke test the two-account flows

## File References For This Fork

- `CONTEXT.md`
- `README.dev.md`
- `FORK_MAINTENANCE.md`
