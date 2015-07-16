use super::super::components::component::Component;

pub struct ConcreteUnit {
  x: u8,
  z: u8
}

impl ConcreteUnit {
  pub fn new(x: u8, z: u8) -> ConcreteUnit {
    ConcreteUnit {
      x: x,
      z: z
    }
  }

  pub fn move_to(&mut self, x: i8, z: i8) -> (u8, u8) {
    if (x < 0) { self.x -= -x as u8; } else { self.x += x as u8; }
    if (z < 0) { self.z -= -z as u8; } else { self.z += z as u8; }
    (self.x, self.z)
  }
}



pub struct Unit {
  components: Vec<Box<Component>>,
  concrete: ConcreteUnit
}

impl Unit {
  pub fn new(x: u8, z: u8) -> Unit {
    Unit {
      components: Vec::new(),
      concrete: ConcreteUnit::new(x, z)
    }
  }

  pub fn add_component(&mut self, boxed_component: Box<Component>) {
    self.components.push(boxed_component);
  }

  pub fn clear_components(&mut self) {
    self.components.clear();
  }

  pub fn update(&mut self) {
    for component in self.components.iter_mut() {
      (*component).update(&mut self.concrete);
    }
  }
}
