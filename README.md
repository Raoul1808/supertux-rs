# SuperTux in Rust

This is the git repo for the unofficial fanmade port of SuperTux Milestone 1 (versions 0.1.x) written in Rust.

This project was made solely to learn Rust and WGPU, and to give myself something to do during vacation.

This project is designed to run on both desktop platforms and the web using WebAssembly.

#### Expect a low, beginner-like code quality, as this is my very first big project made in Rust.

## Current state and roadmap

The engine for the game is currently under development. Here is a roadmap of what's done and what's left to do:
- [x] Window creation
- [x] Creation of a rendering context
- [x] Rendering sprites
- [x] Camera
- [ ] Optimized sprite rendering
- [ ] Audio playback
- [ ] Input management

Once the engine grows large enough, I might split it into its own separate crate for personal use in many other projects.

## Building

Pre-requisites:
- Rust
- Cargo
- Trunk (for web platforms)
- Something to host an HTTP server (for web platforms. Trunk does this for you)

For desktop platforms:
1. Clone this repo and `cd` into it
2. Run `cargo build` (or `cargo run` if you wish to run this immediately)
3. If you are building the project in release mode, remember to copy the `res` directory in the target/release directory.

For web platforms:
1. Download trunk from cargo
2. Clone this repo and `cd` into it
3. Run `trunk build`
4. Run an HTTP server in the `dist` directory. If you are using trunk, you can use the command `trunk serve` to host a web server directly.

## License

This project is licensed under the GPLv3 license.
