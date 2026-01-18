# Repository Guidelines

## Project Structure & Module Organization
- `src/` holds the compositor implementation (`main.rs`, rendering, layout, state, handlers).
- `test-client/` is a small Wayland client used for local smoke testing.
- `assets/` contains runtime assets such as icons.
- `vendor/` contains vendored Rust dependencies (do not edit unless updating vendoring).
- `build.rs` handles build-time checks; `run_waypipe.sh` helps connect external clients.

## Build, Test, and Development Commands
- `cargo build --release` builds the compositor binary for production use.
- `cargo run --release` runs the compositor (`cocoa-way`) and creates the Wayland socket.
- `cargo run --bin test-client` runs the local test client in `test-client/`.
- `./run_waypipe.sh ssh user@host <program>` connects a remote Wayland client via waypipe.
- `cargo test` runs Rust unit tests (none are currently defined in the main crate).

## Coding Style & Naming Conventions
- Rust formatting follows standard rustfmt defaults (4-space indentation, trailing commas).
- Use `snake_case` for functions and modules, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants.
- Keep modules small and focused; prefer adding new files under `src/` rather than large monoliths.

## Testing Guidelines
- The project relies on Rust’s built-in test harness (`cargo test`).
- Use `test-client` for manual validation of input, rendering, and socket connectivity.
- If you add tests, keep them close to the module they cover and use descriptive names like `renders_focus_ring`.

## Commit & Pull Request Guidelines
- Follow the existing commit pattern: short, imperative messages with optional prefixes like `docs:`, `build:`, or `feat:`.
- PRs should describe the change, list test commands run, and link related issues.
- Include screenshots or short clips for visual or rendering changes.

## Security & Configuration Tips
- The compositor expects `libxkbcommon`, `pixman`, and `pkg-config` on macOS; document any new system dependencies in `README.md`.
- Avoid hard-coding paths; rely on `XDG_RUNTIME_DIR` and `WAYLAND_DISPLAY` where applicable.
