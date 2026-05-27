//! Jarsdraw demo — WASM entry point for the browser demo application.
//!
//! Exposes [`JarsdrawDemo`] to JavaScript, which wraps a `<canvas>` element
//! and handles drawing operations triggered by user interaction.
use core::f64;
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
        Ok(Self { canvas, ctx })
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

    /// Clears the entire canvas.
    pub fn clear(&self) {
        web_sys::console::log_1(&"Clearing canvas".into());
        self.ctx.clear_rect(0.0, 0.0, self.width(), self.height());
    }

    /// Draws a filled black circle of radius 10 at the given canvas coordinates.
    pub fn click(&self, x: u32, y: u32) {
        web_sys::console::log_1(&format!("Drawing shape at ({x}, {y})").into());
        self.ctx.set_fill_style_str("black");
        self.ctx.begin_path();
        self.ctx.arc(x as f64, y as f64, 10.0, 0.0, f64::consts::TAU).unwrap();
        self.ctx.fill();
    }
}
