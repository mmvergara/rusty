use std::fmt::Display;

use super::area::Area;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            height: 0.0,
            width: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({},{}): {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}
