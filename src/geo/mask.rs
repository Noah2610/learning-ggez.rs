use ::geo::{Point, Size};

pub enum Origin {
  Center,
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

enum Side {
  Top(f32),
  Bottom(f32),
  Left(f32),
  Right(f32),
}

struct SideCollection {
  top:    f32,
  bottom: f32,
  left:   f32,
  right:  f32
}

impl SideCollection {
  pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
    Self { top, bottom, left, right }
  }
}

// MASK //
pub struct Mask {
  pub point:  Point,
  pub size:   Size,
  pub origin: Origin,
}

impl Mask {
  pub fn new(x: f32, y: f32, w: f32, h: f32, origin: Origin) -> Self {
    Self {
      point: Point { x, y },
      size:  Size  { w, h },
      origin
    }
  }

  pub fn point(&self) -> &Point {
    &self.point
  }

  pub fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }

  pub fn set_point(&mut self, point: Point) {
    self.point = point;
  }

  pub fn rectangle(&self) -> ::ggez::graphics::Rect {
    let top_left: Point = self.top_left();
    return [
      top_left.x,  top_left.y,
      self.size.w, self.size.h
    ].into();
  }

  pub fn intersects(&self, mask: &Mask) -> bool {
    let self_sides: SideCollection = self.sides();
    let othr_sides: SideCollection = mask.sides();

    return (
      (
        (
          self_sides.left >= othr_sides.left &&
          self_sides.left <= othr_sides.right
        ) || (
          self_sides.left  <= othr_sides.left &&
          self_sides.right >= othr_sides.left
        )
      ) && (
        (
          self_sides.top >= othr_sides.top &&
          self_sides.top <= othr_sides.bottom
        ) || (
          self_sides.top    <= othr_sides.top &&
          self_sides.bottom >= othr_sides.top
        )
      )
    );
  }

  fn top_left(&self) -> Point {
    match self.origin {
      Origin::Center => Point::new(
        self.point.x - self.size.w / 2.0,
        self.point.y - self.size.h / 2.0
      ),
      Origin::TopLeft => Point::new(
        self.point.x,
        self.point.y
      ),
      Origin::TopRight => Point::new(
        self.point.x - self.size.w,
        self.point.y
      ),
      Origin::BottomLeft => Point::new(
        self.point.x,
        self.point.y - self.size.h
      ),
      Origin::BottomRight => Point::new(
        self.point.x - self.size.w,
        self.point.y - self.size.h
      )
    }
  }

  fn side(&self, side: char) -> f32 {
    let top_left: Point = self.top_left();
    return match side {
      't' => top_left.y,
      'b' => top_left.y + self.size.h,
      'l' => top_left.x,
      'r' => top_left.x + self.size.w,
      _   => panic!["geo::Mask::side() expected one of 't', 'b', 'l', or 'r'"],
    };
  }

  fn sides(&self) -> SideCollection {
    SideCollection::new(
      self.side('t'),
      self.side('b'),
      self.side('l'),
      self.side('r')
    )
  }
}
