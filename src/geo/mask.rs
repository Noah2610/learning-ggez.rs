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
  point:  Point,
  size:   Size,
  origin: Origin,
}

impl Mask {
  pub fn new(x: f32, y: f32, w: f32, h: f32, origin: Origin) -> Self {
    Self {
      point: Point { x, y },
      size:  Size  { w, h },
      origin
    }
  }

  pub fn rectangle(&self) -> ::ggez::graphics::Rect {
    [
      self.point.x, self.point.y,
      self.size.w,  self.size.h
    ].into()
  }

  fn side(&self, side: char) -> f32 {
    let top_left: Point = match self.origin {
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
    };

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

  fn intersects(&self, mask: Mask) -> bool {
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
}
