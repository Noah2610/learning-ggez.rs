use ::ggez;
use ::ggez::{Context, GameResult};
use ::settings::game::WINDOW_SIZE;
use ::entity::{Player, Wall};

pub struct GameManager {
  player: Player,
  walls:  Vec<Wall>
}

impl GameManager {
  pub fn new() -> Self {
    let player: Player = Player::new(WINDOW_SIZE.w / 2.0, WINDOW_SIZE.h / 2.0);
    let walls:  Vec<Wall> = vec![
      // Wall::new(0, 0)
    ];
    return Self {
      player,
      walls
    }
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &mut self.walls
    { wall.update(ctx)?; }
    self.player.update(ctx)?;

    return Ok(());
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &mut self.walls
    { wall.draw(ctx)?; }
    self.player.draw(ctx)?;

    return Ok(());
  }
}
