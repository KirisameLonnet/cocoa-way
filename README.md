# Cocoa Way

A proof-of-concept Wayland compositor for macOS, built with Rust, [Smithay](https://github.com/Smithay/smithay), and [winit](https://github.com/rust-windowing/winit).

This project allows running Wayland clients (from a remote Linux machine via `waypipe` or locally) and displaying them natively on macOS windows. It serves as a modern, high-performance **alternative to XQuartz (X11)** for running Linux GUI applications on Mac.

> [!WARNING]
> This is an **early Alpha version**. It is currently **unstable** and may experience **high latency**. Use for testing and development purposes only.

## Features & Highlights
- **Native macOS Integration**: Runs as a standard macOS application using `winit`.
- **Seamless Remote Apps**: Stream Linux applications to macOS over SSH using **waypipe-darwin** (optimized for macOS).
- **High Performance**: Uses shared memory (SHM) for low-latency buffer updates.
- **HiDPI / Retina Ready**: Correctly handles scaling for crisp text on MacBook displays.
- **Modern Stack**: Built with **Rust**, **Smithay**, and **Wayland** protocols.

## Keywords used for Search
macOS Wayland Compositor, Run Linux Apps on Mac, XQuartz Alternative, Waypipe macOS, Rust Wayland, Smithay Example, Remote Linux GUI, Retina Wayland, SSH X11 Forwarding Alternative.

## Prerequisities

- Rust (latest stable)
- `pkg-config` (for waypipe build)

## Build & Run

### 1. Build the Compositor
```bash
cargo run --release
```

### 2. Install Waypipe-Darwin
You need the specialized `waypipe-darwin` binary to handle connections.

1.  Download/Clone the **waypipe-darwin** repository.
2.  Build it: `cargo build --release --no-default-features`
3.  Ensure the resulting binary is named `waypipe` and is in your system `$PATH` (e.g., copy to `/usr/local/bin`).

### 3. Connect a Client
Keep the compositor running. Open a new terminal and use the provided wrapper script to connect to your Linux machine.

> [!TIP]
> If the script refuses to run, grant it execution permissions first:
> `chmod +x run_waypipe.sh`

```bash
# Example: Launch Falkon
./run_waypipe.sh ssh user@your-server-ip <Program Name>
```

## Structure
- `src/`: Compositor source code.
- `run_waypipe.sh`: Helper script to set up environment (`XDG_RUNTIME_DIR`) and launch `waypipe`.

## License

**All rights reserved.**

No license is granted for the use, modification, or distribution of this software. You may view the source code for educational purposes, but you may not use it in any commercial or non-commercial projects without explicit written permission from the author.
