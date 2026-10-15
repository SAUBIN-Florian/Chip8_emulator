# Chip8 Emulator in Rust

![Emulator example](./preface.png)

This project was created in the goal of exploring how emulator works and how to simulate a full fledge CPU.

## Features

- Can read & run .ch8 games
- Emulate Chip8 gamestation
- Buffer from the CPU's instructions to a graphic pipeline

## Crates

- winit
- wgpu
- rand
- anyhow

## Build

This example assume you have `cargo edition 2024` installed on your machine:

```bash
cargo build
cargo run
```

## License

This project is for learning and experimentation purposes.
