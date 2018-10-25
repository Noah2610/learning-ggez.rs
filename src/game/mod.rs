use ::std::time::{Instant, Duration};
use ::ggez;
use ::ggez::{
  graphics,
  Context,
  GameResult,
  event
};
use ::settings::game::*;
use ::entity;

mod game_manager;

// GAME //
pub struct Game {
  ctx:        ggez::Context,
  game_state: GameState
}

impl Game {
  pub fn new() -> Self {
    let ctx: Context = ggez::ContextBuilder::new(
      TITLE, AUTHOR
    ).window_setup(
      ggez::conf::WindowSetup::default().title(&WINDOW_TITLE)
    ).window_mode(
      ggez::conf::WindowMode::default().dimensions(
        WINDOW_SIZE.w as u32,
        WINDOW_SIZE.h as u32
      )
    ).build().expect("Failed to build ggez context");

    Self {
      ctx,
      game_state: GameState::new()
    }
  }

  pub fn run(&mut self) {
    graphics::set_background_color(&mut self.ctx, BG_COLOR.into());
    self.game_state.last_update = Instant::now();
    match event::run(&mut self.ctx, &mut self.game_state) {
      Err(e) => println!("Error encountered running game:\n{:#?}", e),
      Ok(_)  => println!("Exit game!")
    }
  }
}

// GAME_STATE //
struct GameState {
  game_manager: game_manager::GameManager,
  last_update:  Instant
}

impl GameState {
  pub fn new() -> Self {
    Self {
      game_manager: game_manager::GameManager::new(),
      last_update:  Instant::now()
    }
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if Instant::now() - self.last_update < Duration::from_millis(UPDATE_DELAY)
    { return Ok(()); }

    self.game_manager.update(_ctx)?;

    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Clear the screen
    graphics::clear(ctx);

    self.game_manager.draw(ctx)?;

    // Actually draw the new frame
    graphics::present(ctx);
    // "We yield the current thread until the next update" (?)
    ggez::timer::yield_now();
    return Ok(());
  }
}
