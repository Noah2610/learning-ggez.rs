extern crate ggez;
// #[macro_use] extern crate maplit;

//use std::collections::HashMap;

mod settings;
mod geo;
mod color;
mod entity;
mod control;
mod game;

fn main() {
  let mut game = game::Game::new();
  game.run();
}
