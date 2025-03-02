use std::{f64::consts::PI, fmt::Display};

use super::area::Area;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
        }
    }
}


