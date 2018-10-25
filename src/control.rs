use ::ggez::event::Keycode;

pub enum ControlType {
  Up,
  Down,
  Left,
  Right
}

pub struct Control {
  control_type: ControlType,
  keycodes:     Vec<Keycode>
}

impl Control {
  pub fn new(control_type: ControlType, keycodes: &Vec<Keycode>) -> Self {
    Self {
      control_type,
      keycodes: keycodes.to_vec()
    }
  }

  pub fn is(&self, keycode: &Keycode) -> bool {
    self.keycodes.contains(keycode)
  }
}

pub struct MovementControls {
  controls: Vec<Control>
}

impl MovementControls {
  pub fn new(up: &Vec<Keycode>, down: &Vec<Keycode>, left: &Vec<Keycode>, right: &Vec<Keycode>) -> Self {
    Self {
      controls: vec![
        Control::new(ControlType::Up, up),
        Control::new(ControlType::Down, down),
        Control::new(ControlType::Left, left),
        Control::new(ControlType::Right, right)
      ]
    }
  }

  pub fn find(&self, keycode: &Keycode) -> Option<&ControlType> {
    if let Some(control) = self.controls.iter().find( |&control| control.is(keycode) ) {
      Some(&control.control_type)
    } else { None }
  }
}
