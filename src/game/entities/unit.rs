use super::super::components::component::Component;

pub struct Unit {
  x: u8,
  z: u8,
  components: Vec<Box<Component>>
}

impl Unit {
  pub fn new(x: u8, z: u8) -> Unit {
    Unit {
      x: x,
      z: z,
      components: Vec::new()
    }
  }

  pub fn add_component(&mut self, boxed_component: Box<Component>) {
    self.components.push(boxed_component);
  }

  pub fn clear_components(&mut self) {
    self.components.clear();
  }

  pub fn update(&mut self) {
    // FIXME: Broken code?
    // for component in self.components.iter_mut() {
    //   (*component).update(self);
    // }
  }
}
