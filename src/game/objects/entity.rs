use super::super::components::component::Component;

pub struct ConcreteEntity {
  x: u8,
  z: u8
}

impl ConcreteEntity {
  pub fn new(x: u8, z: u8) -> ConcreteEntity {
    ConcreteEntity {
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



pub struct Entity {
  components: Vec<Box<Component>>,
  concrete: ConcreteEntity
}

impl Entity {
  pub fn new(x: u8, z: u8) -> Entity {
    Entity {
      components: Vec::new(),
      concrete: ConcreteEntity::new(x, z)
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



#[test]
fn sequential_moves() {
  let mut entity = Entity::new(0, 0);
  entity.concrete.move_to(5, 2);
  entity.concrete.move_to(-3, -1);
  let (mut x, mut z) = entity.concrete.move_to(1, 10);
  assert_eq!(x, 3);
  assert_eq!(z, 11);
}
