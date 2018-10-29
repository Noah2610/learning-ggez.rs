use ::ggez::{
  Context,
  GameResult,
  event::Keycode
};
use ::settings::game::WINDOW_SIZE;
use ::entity::{
  Player,
  Wall,
  Entity,
  Solid
};
use ::geo::{Point};

pub trait EntityManager {

}

pub struct GameManager {
  player: Player,
  walls:  Vec<Wall>
}

impl EntityManager for GameManager {}

impl GameManager {
  pub fn new() -> Self {
    let player: Player = Player::new(WINDOW_SIZE.w / 2.0, WINDOW_SIZE.h / 2.0);
    let wall_size = ::settings::entity::wall::SIZE;
    let offset_x: f32 = 64.0;
    let offset_y: f32 = 128.0;
    let mut walls:  Vec<Wall> = (0 .. 10).map( |index| {
      Wall::new(
        offset_x + index as f32 * wall_size.w,
        offset_y
      )
    }).collect::<Vec<Wall>>();
    walls.append(&mut (0 .. 10).map( |index| {
      Wall::new(
        offset_x,
        offset_y + index as f32 * wall_size.h
      )
    }).collect::<Vec<Wall>>());
    return Self {
      player,
      walls
    }
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {
    for keycode in keycodes {
      self.player.key_pressed(keycode);
    }
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &mut self.walls {
      wall.update(ctx)?;
    }

    //let prev_point: Point = self.player.mask().point().clone();
    self.player.update(ctx)?;
    // if [prev_point.x, prev_point.y] == [self.player.mask.point.x, self.player.mask.point.y] {
    //   println!("SAME");
    // }

    // let intersects: bool = self.walls.iter().any( |wall| self.player.intersects(wall));
    // if intersects {
    //   self.player.set_point(prev_point);
    // }

    return Ok(());
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &mut self.walls
    { wall.draw(ctx)?; }
    self.player.draw(ctx)?;

    return Ok(());
  }
}
