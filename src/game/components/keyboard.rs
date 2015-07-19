use super::super::entities::unit::ConcreteUnit;
use super::component::Component;

pub struct KeyboardController;

impl KeyboardController {
  pub fn set_layout(&mut self) {}
}

impl Component for KeyboardController {
  fn update(&mut self, unit: &mut ConcreteUnit) {
    // Put some real controller code here.

  }
}
