use ::ggez::{graphics, Context, GameResult};
use ::settings::entity::{player::*, wall::*};
use ::geo::mask::{Mask, Origin};
use ::color::Color;

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
  entity: Entity
}

impl Player {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      entity: Entity::new(
                Mask::new(
                  x, y,
                  PLAYER_SIZE.w, PLAYER_SIZE.h,
                  Origin::Center
                ),
                PLAYER_COLOR
              ),
    }
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
