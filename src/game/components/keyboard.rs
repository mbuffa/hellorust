use super::super::entities::unit::Unit;
use super::component::Component;

struct KeyboardController;

impl KeyboardController {
  pub fn set_layout(&mut self) {}
}

impl Component for KeyboardController {
  fn update(&mut self, unit: &mut Unit) {
    // Put some controller code here.
  }
}
