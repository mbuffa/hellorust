use game::entities::map::Map;
use game::entities::unit::Unit;
use game::components::keyboard::KeyboardController;

pub struct Master {
  map: Map,
  units: Vec<Unit>
}

impl Master {
  pub fn new() -> Master {
    let mut master = Master {
      map: Map::new(32, 32),
      units: Vec::new()
    };

    let mut unit = Unit::new(0, 0);
    unit.add_component(Box::new(KeyboardController));
    master.units.push(unit);

    master
  }

  pub fn update(&mut self) {
    println!("Game Master Updating.");

    for unit in self.units.iter_mut() {
      unit.update();
    }
  }
}