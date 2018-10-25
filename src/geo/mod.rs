// POINT //
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  pub fn add(&mut self, move_by_point: &Point) {
    self.x += move_by_point.x;
    self.y += move_by_point.y;
  }
}

// SIZE //
pub struct Size {
  pub w: f32,
  pub h: f32,
}

impl Size {
  pub fn new(w: f32, h: f32) -> Self {
    Self { w, h }
  }
}

pub mod mask;
