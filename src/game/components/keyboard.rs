use super::super::objects::entity::ConcreteEntity;
use super::component::Component;

pub struct KeyboardController;

impl KeyboardController {
  pub fn set_layout(&mut self) {}
}

impl Component for KeyboardController {
  fn update(&mut self, entity: &mut ConcreteEntity) {
    // Put some real controller code here.

  }
}
