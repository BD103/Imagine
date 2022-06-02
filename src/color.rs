use std::fmt;

/// Represents an RGB value.
pub type Rgb = lodepng::RGB<u8>;

/// Creates a new RGB color.
#[inline]
pub fn rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb::new(r, g, b)
}

/// Returns the HSL lightness of a color.
pub fn get_lightness(color: Rgb) -> f32 {
    color.r.max(color.g.max(color.b)) as f32 / 255.0
}

/// Returns true if color is darker.
pub fn is_dark(color: Rgb) -> bool {
    if get_lightness(color) < 0.5 {
        true
    } else {
        false
    }
}

/// A palette of colors.
pub struct Palette {
    colors: Vec<Rgb>,
    index: usize,
}

impl Palette {
    /// Creates a new palette.
    pub fn new(colors: Vec<Rgb>) -> Self {
        Palette { colors, index: 0 }
    }

    /// Gets the current selected color.
    pub fn get(&self) -> Rgb {
        self.colors[self.index]
    }

    /// Selects a given color based off of an index.
    pub fn select(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.colors.len() {
            self.index = index;
            Ok(())
        } else {
            Err("Index out-of-bounds.")
        }
    }
}

impl fmt::Display for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use termion::color::*;

        let mut count = 0;

        // Jumbled mess used to display palette through an iterator
        write!(
            f,
            "{}",
            self.colors
                .iter()
                .map(|x| {
                    count += 1;

                    format!(
                        "{}{}{}{} {}",
                        Bg(Rgb(x.r, x.g, x.b)),
                        {
                            if is_dark(*x) {
                                White.fg_str()
                            } else {
                                Black.fg_str()
                            }
                        },
                        count,
                        Fg(Reset),
                        Bg(Reset),
                    )
                })
                .collect::<String>()
        )
    }
}
