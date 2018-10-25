use ::ggez::event::Keycode;

type ControlArray = [Keycode; 3];

pub const UP: ControlArray = [
  Keycode::Up,
  Keycode::W,
  Keycode::K
];
pub const DOWN: ControlArray = [
  Keycode::Down,
  Keycode::S,
  Keycode::J
];
pub const LEFT: ControlArray = [
  Keycode::Left,
  Keycode::A,
  Keycode::H
];
pub const RIGHT: ControlArray = [
  Keycode::Right,
  Keycode::D,
  Keycode::L
];
