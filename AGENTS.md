# Repository Guidelines

## Project Structure & Module Organization
- `src/`: React + TypeScript UI (`App.tsx`, `main.tsx`, styles/assets under `src/assets/`).
- `src-tauri/`: Tauri (Rust) backend. Entry: `src-tauri/src/main.rs`; app logic in `src-tauri/src/lib.rs` (window events, commands like `greet`, `set_always_on_top`). Config: `src-tauri/tauri.conf.json`. Icons under `src-tauri/icons/`.
- Root config: `package.json`, `vite.config.ts`, `index.html`, `pnpm-lock.yaml`, `public/`.

## Build, Test, and Development Commands
- Frontend dev: `pnpm dev` (Vite on port 1420).
- Desktop app dev: `pnpm run tauri dev` (runs Vite + Tauri shell).
- Frontend build: `pnpm build` (type-checks with `tsc` and bundles).
- Desktop build: `pnpm run tauri build` (creates platform installers/binaries).
- Rust only (optional): `cargo build` / `cargo test` inside `src-tauri/`.

## Coding Style & Naming Conventions
- Rust: run `cargo fmt` and `cargo clippy -D warnings` in `src-tauri/` before committing.
- TypeScript/React: 2-space indent, camelCase for variables/functions, PascalCase for components/files (e.g., `MyPanel.tsx`). Keep JSX minimal and typed; prefer `invoke` from `@tauri-apps/api` for IPC.
- Files/paths: co-locate component styles; keep imports relative and stable.

## Testing Guidelines
- No formal framework configured yet. If adding tests:
  - Rust: add `#[test]` units in the same module or `src-tauri/tests/`; run `cargo test`.
  - Frontend: place `*.test.ts(x)` next to code and use Vitest or similar; add an npm script to run them.

## Commit & Pull Request Guidelines
- Use Conventional Commits (e.g., `feat: add always-on-top toggle`, `fix: restore aspect ratio on resize`).
- PRs should include: clear description, linked issues (`Closes #123`), screenshots/gifs for UI changes, and platforms tested (macOS/Windows/Linux). Ensure `pnpm build` and `pnpm run tauri build` succeed locally.

## Architecture Overview & Tips
- UI calls Rust commands via `@tauri-apps/api` `invoke` (see `set_always_on_top`).
- Backend listens to window resize events and enforces aspect ratio in `src-tauri/src/lib.rs`.
- Tauri config uses Vite on `1420` (`vite.config.ts`) and bundles from `dist/`. Review `security.csp` before release and scope capabilities/plugins conservatively.
