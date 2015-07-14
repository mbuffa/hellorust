use game::entities::map;

pub struct Master {
  map: map::Map
}

impl Master {
  pub fn new() -> Master {
    Master {
      map: map::Map::new(32, 32)
    }
  }

  pub fn update(&mut self) {
    println!("Game Master Updating.");
    println!("Initialized? {}", self.map.is_initialized());
    println!("Walkable? {}", self.map.tile_at(1, 1).is_walkable());
  }
}