use std::fs;

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
      if let Color::Green = self {
        return true;
      }
      return false;
    }
}

fn main() {}
