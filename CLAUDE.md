# Jarsdraw

A Rust/WebAssembly canvas drawing library for the browser (edition 2024), compiled via
`wasm-bindgen`/`web-sys`. See [README.md](README.md) for installation and usage.

## Structure

Two Cargo crates in this repo:

- **`src/`** — the `jarsdraw` library (`cdylib` + `rlib`).
  - `canvas.rs` — `Canvas`: wraps an `HtmlCanvasElement` + `CanvasRenderingContext2d`;
    `Canvas::from_selector` binds to a DOM element, `Canvas::draw` dispatches to a `Draw`.
  - `draw.rs` — `Draw` trait: implemented by anything that can render itself onto a `Canvas`.
  - `polyline.rs` — `Polyline`: a `Draw` primitive rendering a connected sequence of points.
  - `styled.rs` — `Style`/`Shape`/`Styled`: chainable stroke styling (`.into_styled().stroke(..).line_width(..)`)
    layered on top of any `Shape` (a `Draw` + `Sized` type).
  - `lib.rs` — re-exports the public API: `Canvas`, `Draw`, `Polyline`, `Shape`, `Style`, `Styled`.
- **`demo/`** — a separate `jarsdraw-demo` crate plus a webpack/JS front end (`demo/index.js`,
  `demo/index.html`) that exercises the library: click-to-add-point polyline drawing on a
  resizable canvas, wired up via `wasm-bindgen`.

## Conventions

- Every public function/type/trait carries a `///` docstring with `# Parameters`, `# Returns`,
  `# Errors`/`# Panics`, and `# Side Effects` sections where applicable (see any file in `src/`
  for the pattern). Trait methods and their impls are each documented individually — the impl's
  doc describes what that specific implementation does.
- Keep this file and `README.md` in sync with `src/lib.rs`'s module list whenever a module is
  added, renamed, or removed.

## Commands

- `cargo check` / `cargo check` (from `demo/`) — type-check both crates.
- `wasm-pack build --target bundler` — build the library crate to `pkg/`.
- `wasm-pack test --firefox` — run the library's wasm-bindgen tests.
- `cd demo && npm start` — build the Rust WASM, watch for changes, and serve the demo page.
