use ::ggez::{
  graphics,
  Context,
  GameResult,
  event::Keycode
};
use ::settings::entity::{player, wall};
use ::geo::Point;
use ::geo::mask::{Mask, Origin};
use ::color::Color;
use ::control::{MovementControls, ControlType};
use ::game::game_manager::{GameManager, EntityManager};

pub trait Entity {
  fn mask(&self) -> &Mask;
  fn mask_mut(&mut self) -> &mut Mask;
  fn color(&self) -> &Color;

  fn set_point(&mut self, point: Point) {
    self.mask_mut().set_point(point);
  }

  // fn move_by(&mut self, move_point: &Point) {
  //   self.mask_mut().point_mut().add(&move_point);
  // }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    return Ok(());
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::set_color(ctx, (*self.color()).into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, self.mask().rectangle())?;
    //graphics::rectangle(ctx, graphics::DrawMode::Fill, [0.0f32, 0.0f32, 0.0f32, 0.0f32].into());
    return Ok(());
  }
}

pub trait Solid: Entity {
  fn move_by(&mut self, move_point: &Point) {
    self.mask_mut().point_mut().add(&move_point);
  }

  fn intersects<E: Entity>(&self, other: &E) -> bool {
    self.mask().intersects(other.mask())
  }
}

// PLAYER //
pub struct Player {
  pub mask:     Mask,
  color:        Color,
  controls:     MovementControls
}

impl Entity for Player {
  fn mask(&self) -> &Mask {
    return &self.mask;
  }

  fn mask_mut(&mut self) -> &mut Mask {
    return &mut self.mask;
  }

  fn color(&self) -> &Color {
    return &self.color;
  }
}

impl Solid for Player {
}

impl Player {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      color: player::COLOR,
      controls: MovementControls::new(
        &player::controls::UP.to_vec(),
        &player::controls::DOWN.to_vec(),
        &player::controls::LEFT.to_vec(),
        &player::controls::RIGHT.to_vec()
      ),
      mask: Mask::new(
        x, y,
        player::SIZE.w, player::SIZE.h,
        Origin::Center
      )
    }
  }

  pub fn key_pressed(&mut self, keycode: &Keycode) {
    let mut mv_opt: Option<Point> = None;
    if let Some(dir) = self.controls.find(keycode) {
      match dir {
        ControlType::Up    => mv_opt = Some(Point::new(0.0, -player::STEP_SIZE)),
        ControlType::Down  => mv_opt = Some(Point::new(0.0,  player::STEP_SIZE)),
        ControlType::Left  => mv_opt = Some(Point::new(-player::STEP_SIZE, 0.0)),
        ControlType::Right => mv_opt = Some(Point::new( player::STEP_SIZE, 0.0))
      }
    }
    if let Some(mv) = mv_opt {
      self.move_by(&mv);
    }
  }
}

// WALL //
pub struct Wall {
  mask:  Mask,
  color: Color
}

impl Entity for Wall {
  fn mask(&self) -> &Mask {
    return &self.mask;
  }

  fn mask_mut(&mut self) -> &mut Mask {
    return &mut self.mask;
  }

  fn color(&self) -> &Color {
    return &self.color;
  }
}

impl Wall {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      color: wall::COLOR,
      mask: Mask::new(
        x, y,
        wall::SIZE.w, wall::SIZE.h,
        Origin::TopLeft
      )
    }
  }
}
