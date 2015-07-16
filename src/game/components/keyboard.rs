use super::super::entities::unit::ConcreteUnit;
use super::component::Component;

pub struct KeyboardController;

impl KeyboardController {
  pub fn set_layout(&mut self) {}
}

impl Component for KeyboardController {
  fn update(&mut self, unit: &mut ConcreteUnit) {
    // Put some real controller code here.
    let (mut x, mut z) = unit.move_to(5, 2);
    println!("Unit moved to {} {}", x, z);
    let (mut x, mut z) = unit.move_to(-3, 0);
    println!("Unit moved to {} {}", x, z);
    let (mut x, mut z) = unit.move_to(1, 10);
    println!("Unit moved to {} {}", x, z);
  }
}
