use game::objects::map::Map;
use game::objects::entity::Entity;
use game::components::keyboard::KeyboardController;

pub struct Master {
  map: Map,
  entities: Vec<Entity>
}

impl Master {
  pub fn new() -> Master {
    let mut master = Master {
      map: Map::new(32, 32),
      entities: Vec::new()
    };

    let mut entity = Entity::new(0, 0);
    entity.add_component(Box::new(KeyboardController));
    master.entities.push(entity);

    master
  }

  pub fn update(&mut self) {
    println!("Game Master Updating.");

    for entity in self.entities.iter_mut() {
      entity.update();
    }
  }
}