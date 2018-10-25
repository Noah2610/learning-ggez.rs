use ::ggez::{
  graphics,
  Context,
  GameResult,
  event::Keycode
};
use ::settings::entity::{player::*, wall::*};
use ::geo::Point;
use ::geo::mask::{Mask, Origin};
use ::color::Color;
use ::control::{MovementControls, ControlType};

// ENTITY //
struct Entity {
  mask:  Mask,
  color: Color
}

impl Entity {
  pub fn new(mask: Mask, color: Color) -> Self {
    Self { mask, color }
  }

  pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()>{
    return Ok(());
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
    graphics::set_color(ctx, self.color.into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, self.mask.rectangle())?;
    //graphics::rectangle(ctx, graphics::DrawMode::Fill, [0.0f32, 0.0f32, 0.0f32, 0.0f32].into());
    return Ok(());
  }
}

// PLAYER //
pub struct Player {
  filler:   i8,  // NOTE: This fixes a vim auto-formatting bug that _bugs_ me.
  entity:   Entity,
  controls: MovementControls
}

impl Player {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      filler: 0, // NOTE: Fixes a vim auto-formatting bug.
      controls: MovementControls::new(
        &controls::UP.to_vec(),
        &controls::DOWN.to_vec(),
        &controls::LEFT.to_vec(),
        &controls::RIGHT.to_vec()
      ),
      entity: Entity::new(
        Mask::new(
          x, y,
          PLAYER_SIZE.w, PLAYER_SIZE.h,
          Origin::Center
        ),
        PLAYER_COLOR
      )
    }
  }

  pub fn key_pressed(&mut self, keycode: &Keycode) {
    let mut mv_opt: Option<Point> = None;
    if let Some(dir) = self.controls.find(keycode) {
      match dir {
        ControlType::Up    => mv_opt = Some(Point::new(0.0, -STEP_SIZE)),
        ControlType::Down  => mv_opt = Some(Point::new(0.0,  STEP_SIZE)),
        ControlType::Left  => mv_opt = Some(Point::new(-STEP_SIZE, 0.0)),
        ControlType::Right => mv_opt = Some(Point::new( STEP_SIZE, 0.0))
      }
    }
    if let Some(mv) = mv_opt {
      self.move_by(&mv);
    }
  }

  fn move_by(&mut self, move_point: &Point) {
    self.entity.mask.point.add(&move_point);
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
    self.entity.update(ctx)?;
    return Ok(());
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
    self.entity.draw(ctx)?;
    return Ok(());
  }
}

// WALL //
pub struct Wall {
  entity: Entity
}

impl Wall {
  pub fn new(x: f32, y: f32) -> Self {
    return unimplemented!();
    // Self {
    //   entity: Entity {
    //     mask: Mask::new()
    //   }
    // }
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
    self.entity.update(ctx)?;
    return Ok(());
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
    self.entity.draw(ctx)?;
    return Ok(());
  }
}
