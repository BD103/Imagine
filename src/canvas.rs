use crate::color::Rgb;

pub const DEFAULT_PIXEL: Rgb = Rgb::new(239, 239, 239);

#[derive(Debug)]
pub struct Canvas {
    pixels: Vec<Rgb>,

    pub width: usize,
    pub height: usize,
}

impl Canvas {
    /// Creates a new [Canvas] filled with [DEFAUL_PIXEL]s.
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            pixels: vec![DEFAULT_PIXEL; width * height],

            width,
            height,
        }
    }

    /// Returns the RGB value of a pixel at the given coordinates.
    pub fn get(&self, coords: (usize, usize)) -> Rgb {
        self.pixels[coords.0 + coords.1 * self.height]
    }

    /// Returns the RGB value of a pixel based on the linear index.
    pub fn get_linear(&self, index: usize) -> Rgb {
        self.pixels[index]
    }

    pub fn get_buffer(&self) -> &[Rgb] {
        &self.pixels
    }

    /// Paints a specific pixel the given color.
    pub fn paint(&mut self, coords: (usize, usize), color: Rgb) {
        self.pixels[coords.0 + coords.1 * self.height] = color;
    }
}
