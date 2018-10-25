// POINT //
pub struct Point {
  x: f32,
  y: f32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}

// SIZE //
pub struct Size {
  w: f32,
  h: f32,
}

impl Size {
  pub fn new(w: f32, h: f32) -> Self {
    Self { w, h }
  }
}

mod mask;
