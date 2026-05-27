# Jarsdraw

A Rust/WebAssembly canvas drawing library for the browser, built with `wasm-bindgen`.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Adding as a dependency

Add `jarsdraw` to your `Cargo.toml`:

```toml
[dependencies]
jarsdraw = { path = "/path/to/jarsdraw" }
```

## Usage

Build the library with `wasm-pack` targeting your preferred bundler:

```sh
wasm-pack build --target bundler
```

This outputs a `pkg/` directory containing the compiled `.wasm` file and JavaScript bindings, which can be imported in any bundler-based web project.

### Demo Application

The `demo/` directory contains a full browser demo showing canvas drawing via click events.

```sh
cd demo
npm install
npm start
```

This builds the Rust WASM code, starts a file watcher to rebuild on changes, and serves the page with webpack-dev-server.

### Testing

```sh
wasm-pack test --firefox
```
