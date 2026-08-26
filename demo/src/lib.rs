//! Jarsdraw demo — WASM entry point for the browser demo application.
//!
//! Exposes [`JarsdrawDemo`] to JavaScript, which wraps a `<canvas>` element
//! and handles drawing operations triggered by user interaction.
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

/// Initialize console panic hook
#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
}

/// Wraps an HTML canvas element and its 2D rendering context, exposing
/// drawing operations to JavaScript.
#[wasm_bindgen]
pub struct JarsdrawDemo {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    points: Vec<(f64, f64)>,
}

#[wasm_bindgen]
impl JarsdrawDemo {
    /// Creates a new `JarsdrawDemo` bound to the `<canvas>` element with the given DOM `id`.
    ///
    /// Returns an error if the element cannot be found or cast to a canvas.
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str) -> Result<Self, JsValue> {
        let canvas = window()
            .ok_or(JsValue::from("Unable to load window"))?
            .document()
            .ok_or(JsValue::from("Unable to load document"))?
            .get_element_by_id(id)
            .ok_or(JsValue::from(format!("Could not find html element by id \"{id}\"")))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from(format!("Element with id \"{id}\" can not be converted to canvas")))?;
        let ctx = canvas
            .get_context("2d")?
            .ok_or(JsValue::from("2d rendering context unavailable"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from("Unable to convert to CanvasRenderingContext2d"))?;
        Ok(Self { canvas, ctx, points: Vec::new() })
    }

    /// Returns the canvas width in pixels.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 {
        self.canvas.width() as f64
    }

    /// Returns the canvas height in pixels.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 {
        self.canvas.height() as f64
    }

    /// Clears the entire canvas and resets the polyline being drawn.
    pub fn clear(&mut self) {
        web_sys::console::log_1(&"Clearing canvas".into());
        self.ctx.clear_rect(0.0, 0.0, self.width(), self.height());
        self.points.clear();
    }

    /// Adds a point to the polyline at the given canvas coordinates, extending
    /// the drawn line from the previous point (if any).
    pub fn click(&mut self, x: u32, y: u32) {
        web_sys::console::log_1(&format!("Adding point at ({x}, {y})").into());
        let point = (x as f64, y as f64);

        if let Some(&(prev_x, prev_y)) = self.points.last() {
            self.ctx.set_stroke_style_str("black");
            self.ctx.set_line_width(2.0);
            self.ctx.begin_path();
            self.ctx.move_to(prev_x, prev_y);
            self.ctx.line_to(point.0, point.1);
            self.ctx.stroke();
        }

        self.points.push(point);
    }
}
