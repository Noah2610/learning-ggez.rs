pub const TITLE:  &str = "hello";
pub const AUTHOR: &str = "Noah Rosenzweig";

pub const WINDOW_SIZE: ::geo::Size = ::geo::Size { w: 600.0, h: 600.0 };
pub const WINDOW_TITLE: &str = "Hello!";

pub const BG_COLOR: ::color::Color = [0.75, 0.75, 0.75, 1.0];

pub const FPS: f32 = 30.0;
pub const UPDATE_DELAY: u64 = (1.0 / FPS * 1000.0) as u64;
