use super::super::entities::unit::Unit;
use super::component::Component;

struct KeyboardControlledComponent;

impl KeyboardControlledComponent {
  pub fn set_layout(&mut self) {}
}

impl Component for KeyboardControlledComponent {
  fn update(&mut self, unit: &mut Unit) {
    // Put some controller code here.
  }
}
