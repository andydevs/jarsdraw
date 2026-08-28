//! Jarsdraw demo — WASM entry point for the browser demo application.
//!
//! Exposes [`JarsdrawDemo`] to JavaScript, which wraps a `<canvas>` element
//! and handles drawing operations triggered by user interaction.
use jarsdraw::{Canvas, Polyline};
use wasm_bindgen::prelude::*;

/// Initialize console panic hook
#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
}

/// Wraps an HTML canvas element and its 2D rendering context, exposing
/// drawing operations to JavaScript.
#[wasm_bindgen]
pub struct JarsdrawDemo {
    canvas: Canvas,
    points: Vec<(f64, f64)>,
}

#[wasm_bindgen]
impl JarsdrawDemo {
    /// Creates a new `JarsdrawDemo` bound to the `<canvas>` element with the given DOM `id`.
    ///
    /// Returns an error if the element cannot be found or cast to a canvas.
    #[wasm_bindgen(constructor)]
    pub fn new(selector: &str) -> Result<Self, JsValue> {
        let canvas = Canvas::from_selector(selector)?;
        Ok(Self {
            canvas,
            points: Vec::new(),
        })
    }

    /// Clears the entire canvas and resets the polyline being drawn.
    pub fn clear(&mut self) {
        self.canvas.clear();
        self.points.clear();
    }

    /// Adds a point to the polyline at the given canvas coordinates, extending
    /// the drawn line from the previous point (if any).
    pub fn click(&mut self, x: u32, y: u32) {
        web_sys::console::log_1(&format!("Adding point at ({x}, {y})").into());
        self.points.push((x as f64, y as f64));

        web_sys::console::log_1(&format!("Drawing new polygon").into());
        let poly = Polyline::from_points(&self.points);
        self.canvas.ctx().set_stroke_style_str("black");
        self.canvas.ctx().set_line_width(2.0);
        self.canvas.draw(&poly);
    }
}
