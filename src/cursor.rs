/// Represents a cursor on a canvas.
pub struct Cursor {
    pub x: usize,
    pub y: usize,

    width: usize,
    height: usize,
}

impl Cursor {
    /// Returns a new cursor.
    pub fn new(width: usize, height: usize) -> Self {
        Cursor {
            x: 0,
            y: 0,

            width,
            height,
        }
    }

    /// Moves cursor to given coordinates if within canvas.
    pub fn goto(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if x < self.width && y < self.height {
            self.x = x;
            self.y = y;

            Ok(())
        } else {
            Err("Given coordinates out-of-bounds.")
        }
    }

    /// Moves cursor left amount or to edge of canvas.
    pub fn left(&mut self, amount: usize) {
        self.x = self.x.saturating_sub(amount);
    }

    /// Moves cursor right amount or to edge of canvas.
    pub fn right(&mut self, amount: usize) {
        if self.x + amount < self.width {
            self.x += amount;
        } else {
            self.x = self.width - 1;
        }
    }

    /// Moves cursor up amount or to edge of canvas.
    pub fn up(&mut self, amount: usize) {
        self.y = self.y.saturating_sub(amount);
    }

    /// Moves cursor down amount or to edge of canvas.
    pub fn down(&mut self, amount: usize) {
        if self.y + amount < self.height {
            self.y += amount;
        } else {
            self.y = self.height - 1;
        }
    }

    /// Returns a tuple of the cursor's coordinates.
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
