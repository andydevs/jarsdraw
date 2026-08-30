//! A closed square canvas demo.

use jarsdraw::{Canvas, Polyline, Styled};
use wasm_bindgen::prelude::*;

// Minimum margin from screen
const MARGIN: f64 = 0.24;

// Define Corners
const CORNERS: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// Draws a closed square, scaled to fit the canvas.
#[wasm_bindgen]
pub struct SquareCanvas {
    canvas: Canvas,
}

#[wasm_bindgen]
impl SquareCanvas {
    /// Creates a square canvas bound to the `<canvas>` element at `selector`, then draws
    /// the square immediately.
    ///
    /// # Parameters
    /// - `selector`: a CSS selector identifying the target `<canvas>` element.
    ///
    /// # Returns
    /// A `SquareCanvas` with the square already drawn at the canvas's current dimensions.
    ///
    /// # Errors
    /// Returns a `JsValue` error if the element cannot be found or cast to a canvas.
    #[wasm_bindgen(constructor)]
    pub fn new(selector: &str) -> Result<Self, JsValue> {
        let canvas = Canvas::from_selector(selector)?;
        let mut demo = Self { canvas };
        demo.redraw();
        Ok(demo)
    }

    /// Clears this canvas and redraws the square to fit the canvas's current dimensions.
    ///
    /// # Side Effects
    /// Erases the canvas, then strokes a blue square scaled to the canvas's current
    /// width/height. Call this after resizing the `<canvas>` element.
    pub fn redraw(&mut self) {
        self.canvas.clear();

        // Scale
        let (width, height) = self.canvas.dimensions();
        let scale = (1.0 - MARGIN) * (width.min(height) as f64);

        // Offset
        let offset_x = (width as f64 - scale) / 2.0;
        let offset_y = (height as f64 - scale) / 2.0;

        // Get points from corners
        let points: Vec<_> = CORNERS
            .iter()
            .map(|&(x, y)| (x * scale + offset_x, y * scale + offset_y))
            .collect();

        // Draw square
        let styled = Styled::new(Polyline::new(&points).closed(true))
            .stroke("#3b82f6")
            .line_width(3.0);
        self.canvas.draw(&styled);
    }
}
